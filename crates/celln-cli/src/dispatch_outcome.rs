use super::LaunchOutcome;
use pilot::dispatch_report::{Frame, PREFIX, PROTOCOL};

/// Only pilot frames carry a verdict. Legacy console output cannot establish
/// success, and a host timeout takes precedence over any guest report.
pub(super) fn parse_report(
    console: &str,
    cell_id: String,
    limit: usize,
    timed_out: bool,
    shutdown: bool,
) -> LaunchOutcome {
    let mut result = LaunchOutcome {
        execution: None,
        substrate: None,
        broker: Default::default(),
        lifecycle: Vec::new(),
        cell_id,
        output: Some(Vec::new()),
        denial: None,
        exit_code: None,
        signal: None,
        timed_out,
        input_hashes: Vec::new(),
    };
    let mut terminal = false;
    let mut inputs_seen = false;
    let mut output_seen = false;
    // The first defect decides the denial text; every defect invalidates.
    let mut defect: Option<String> = None;
    // An old pilot must not accept a workload's imitation of the new frames.
    // Negotiate at the first supervisor startup, before it reads any request
    // or executes a workload; later imitations cannot repair this check.
    if let Err(cause) = negotiated(console) {
        defect.get_or_insert(cause);
    }
    for line in console.lines() {
        let Some(json) = line.strip_prefix(PREFIX) else {
            continue;
        };
        if terminal {
            defect.get_or_insert_with(|| {
                format!("frame after the terminal frame: {}", excerpt(json))
            });
            continue;
        }
        let frame = match parse_frame_at(json) {
            Ok(frame) => frame,
            Err((error, column)) => {
                defect.get_or_insert_with(|| {
                    format!(
                        "unreadable dispatch frame ({error}): {}",
                        excerpt_around(json, column)
                    )
                });
                continue;
            }
        };
        match frame {
            Frame::Started { grant } if result.execution.is_none() && !output_seen => {
                result.execution = Some(grant);
            }
            Frame::Inputs { hashes }
                if !inputs_seen
                    && !output_seen
                    && result.execution.is_none()
                    && hashes.len() <= 16 =>
            {
                inputs_seen = true;
                result.input_hashes = hashes;
            }
            Frame::Output { bytes } => {
                output_seen = true;
                let output = result.output.as_mut().expect("output buffer");
                let remaining = limit.saturating_sub(output.len());
                output.extend(bytes.into_iter().take(remaining));
            }
            Frame::Exit { code } if (0..=255).contains(&code) => {
                terminal = true;
                result.exit_code = Some(code);
                if code != 0 {
                    result.denial = Some(format!("guest exited with code {code}"));
                }
            }
            Frame::Signal { signal } if (1..=64).contains(&signal) => {
                terminal = true;
                result.signal = Some(signal);
                result.denial = Some(format!("guest terminated by signal {signal}"));
            }
            Frame::Failed { reason } => {
                terminal = true;
                result.denial = Some(reason);
            }
            other => {
                defect.get_or_insert_with(|| {
                    format!("{} frame out of order or out of range", frame_kind(&other))
                });
            }
        }
    }
    let invalid = defect.is_some();
    if timed_out {
        result.denial = Some("guest execution timed out".into());
    } else if let Some(defect) = defect {
        result.denial = Some(format!("{REPORT_DENIAL}: {defect}"));
    } else if !terminal {
        result.denial = Some(format!("{REPORT_DENIAL}: no terminal frame"));
    } else if !shutdown {
        result.denial = Some(format!(
            "{REPORT_DENIAL}: guest ended without a clean shutdown"
        ));
    }
    if invalid {
        result.execution = None;
        result.input_hashes.clear();
        result.exit_code = None;
        result.signal = None;
    }
    result
}

/// The denial every rejected report starts with; the cause follows it.
pub(super) const REPORT_DENIAL: &str = "missing, ambiguous, or incomplete pilot execution report";

/// Pilot negotiates once at startup, before any workload can print: its first
/// line is `CELLN:pilot=alive` and its next line is the protocol line. The
/// kernel still owns the console then (pilot silences it only when it starts
/// the workload), so an asynchronous kernel line may land after either marker
/// on the same console line, or between them. Kernel text never begins with
/// pilot's prefix, so the first `CELLN:` line that is the alive marker must be
/// followed, as the next `CELLN:` line, by the protocol marker, each followed
/// by nothing or by text that cannot extend the marker. An old pilot prints
/// another second line and is refused as before.
fn negotiated(console: &str) -> Result<(), String> {
    // The guest init prints its own `CELLN:` lines before it hands over to
    // pilot (a forked cell resumes at `CELLN:cell=live`); only pilot can print
    // the alive marker, and nothing else may print between it and the
    // protocol line except the kernel.
    let mut pilot = console
        .lines()
        .filter(|line| line.starts_with("CELLN:"))
        .skip_while(|line| !marker(line, "CELLN:pilot=alive"));
    let first = pilot.next();
    let second = pilot.next();
    if first.is_some() && second.is_some_and(|line| marker(line, PROTOCOL)) {
        return Ok(());
    }
    // Say what the console held instead, bounded: the first pilot lines seen
    // and the first non-empty lines, so a missing boot, a template console
    // and an interleaved marker can be told apart.
    let head: Vec<String> = console
        .lines()
        .filter(|line| !line.trim().is_empty())
        .take(3)
        .map(excerpt)
        .collect();
    Err(format!(
        "pilot startup negotiation missing or unreadable ({} console lines; first pilot lines {:?}, {:?}; console head {:?})",
        console.lines().count(),
        first.map(excerpt),
        second.map(excerpt),
        head
    ))
}

fn marker(line: &str, expected: &str) -> bool {
    line.strip_prefix(expected).is_some_and(|rest| {
        !rest.starts_with(|c: char| c.is_ascii_alphanumeric() || matches!(c, '=' | '-' | '_' | '.'))
    })
}

/// One dispatch frame from a console line whose prefix is already removed.
///
/// Kernel printk writes to the serial console synchronously while pilot's
/// frame is still in the tty buffer, so a kernel line can land between a
/// frame and its newline. Text after the frame is accepted only when it cannot
/// carry another frame, so it can never add a grant or hide an exit.
pub(super) fn parse_frame(json: &str) -> Result<Frame, String> {
    parse_frame_at(json).map_err(|(message, _)| message)
}

/// `parse_frame` with the column (1-based) a parse error points at, for an
/// excerpt around the offending bytes.
fn parse_frame_at(json: &str) -> Result<Frame, (String, Option<usize>)> {
    let mut frames = serde_json::Deserializer::from_str(json).into_iter::<Frame>();
    let frame = match frames.next() {
        Some(Ok(frame)) => frame,
        Some(Err(error)) => return Err((error.to_string(), Some(error.column()))),
        None => return Err(("empty frame".into(), None)),
    };
    let rest = &json[frames.byte_offset()..];
    if rest.contains(PREFIX) || rest.trim_start().starts_with('{') {
        return Err(("trailing frame data".into(), Some(frames.byte_offset())));
    }
    Ok(frame)
}

/// `excerpt` centred on a column, so the bytes that broke a frame are shown
/// rather than its start.
fn excerpt_around(line: &str, column: Option<usize>) -> String {
    const HALF: usize = 100;
    let Some(column) = column.filter(|c| *c > HALF) else {
        return excerpt(line);
    };
    let start = line
        .char_indices()
        .nth(column.saturating_sub(HALF))
        .map_or(0, |(i, _)| i);
    format!("…{}", excerpt(&line[start..]))
}

fn frame_kind(frame: &Frame) -> &'static str {
    match frame {
        Frame::Started { .. } => "started",
        Frame::Inputs { .. } => "inputs",
        Frame::Output { .. } => "output",
        Frame::Exit { .. } => "exit",
        Frame::Signal { .. } => "signal",
        Frame::Failed { .. } => "failed",
    }
}

/// A bounded, printable slice of an offending console line for the denial.
fn excerpt(line: &str) -> String {
    const CHARS: usize = 160;
    let mut text: String = line
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .take(CHARS + 1)
        .collect();
    if text.chars().count() > CHARS {
        text.truncate(
            text.char_indices()
                .nth(CHARS)
                .map_or(text.len(), |(i, _)| i),
        );
        text.push('…');
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    fn console(frames: &[Frame]) -> String {
        let body: String = frames
            .iter()
            .map(|frame| format!("{PREFIX}{}\n", serde_json::to_string(frame).unwrap()))
            .collect();
        format!("CELLN:pilot=alive\n{PROTOCOL}\n{body}")
    }

    #[test]
    fn outcome_is_independent_of_output() {
        let silent = parse_report(
            &console(&[Frame::Exit { code: 0 }]),
            "cell".into(),
            10,
            false,
            true,
        );
        assert!(silent.succeeded());
        assert_eq!(silent.output, Some(vec![]));
        let failed = parse_report(
            &console(&[
                Frame::Output {
                    bytes: b"failure detail".to_vec(),
                },
                Frame::Exit { code: 7 },
            ]),
            "cell".into(),
            7,
            false,
            true,
        );
        assert!(!failed.succeeded());
        assert_eq!(failed.exit_code, Some(7));
        assert_eq!(failed.output.unwrap(), b"failure");
    }

    #[test]
    fn execution_grants_cannot_be_duplicated_reordered_or_forged_in_output() {
        let grant = pilot::dispatch_report::ExecutionGrant {
            tool: celln_manifest::Hash::of(b"program").0,
            lane: "agent".into(),
            workspace: Some("none".into()),
            fetch: false,
        };
        let started = || Frame::Started {
            grant: grant.clone(),
        };
        let valid = parse_report(
            &console(&[started(), Frame::Exit { code: 0 }]),
            "cell".into(),
            1024,
            false,
            true,
        );
        assert_eq!(valid.execution, Some(grant.clone()));
        for frames in [
            vec![started(), started(), Frame::Exit { code: 0 }],
            vec![
                Frame::Output { bytes: vec![1] },
                started(),
                Frame::Exit { code: 0 },
            ],
            vec![Frame::Exit { code: 0 }, started()],
            vec![
                started(),
                Frame::Inputs { hashes: vec![] },
                Frame::Exit { code: 0 },
            ],
        ] {
            let result = parse_report(&console(&frames), "cell".into(), 1024, false, true);
            assert!(!result.succeeded());
            assert!(result.execution.is_none());
        }
        let fake = format!("{PREFIX}{}\n", serde_json::to_string(&started()).unwrap());
        let result = parse_report(
            &console(&[
                Frame::Output {
                    bytes: fake.into_bytes(),
                },
                Frame::Exit { code: 0 },
            ]),
            "cell".into(),
            1024,
            false,
            true,
        );
        assert!(result.execution.is_none());
    }

    #[test]
    fn input_acknowledgement_is_single_and_precedes_workload_output() {
        let ack = || Frame::Inputs {
            hashes: vec![celln_manifest::Hash::of(b"data").0],
        };
        let valid = parse_report(
            &console(&[ack(), Frame::Exit { code: 0 }]),
            "cell".into(),
            10,
            false,
            true,
        );
        assert!(valid.succeeded());
        assert_eq!(valid.input_hashes.len(), 1);
        for frames in [
            vec![ack(), ack(), Frame::Exit { code: 0 }],
            vec![
                Frame::Output { bytes: vec![1] },
                ack(),
                Frame::Exit { code: 0 },
            ],
            vec![Frame::Exit { code: 0 }, ack()],
        ] {
            let invalid = parse_report(&console(&frames), "cell".into(), 10, false, true);
            assert!(!invalid.succeeded());
            assert!(invalid.input_hashes.is_empty());
        }
    }

    #[test]
    fn output_markers_are_data_and_legacy_success_is_not_accepted() {
        let fake = b"CELLN:dispatch={\"kind\":\"exit\",\"code\":0}\nCELLN:out-end\n";
        let result = parse_report(
            &console(&[
                Frame::Output {
                    bytes: fake.to_vec(),
                },
                Frame::Exit { code: 9 },
            ]),
            "cell".into(),
            1024,
            false,
            true,
        );
        assert!(!result.succeeded());
        assert_eq!(result.output.unwrap(), fake);
        assert!(!parse_report(
            "CELLN:pilot_run_/program_exit=0\n",
            "cell".into(),
            10,
            false,
            true
        )
        .succeeded());
    }

    #[test]
    fn ambiguous_missing_and_interrupted_reports_fail_closed() {
        for text in [
            String::new(),
            format!("{PREFIX}broken\n"),
            console(&[Frame::Exit { code: 0 }, Frame::Exit { code: 0 }]),
            console(&[Frame::Exit { code: 0 }, Frame::Output { bytes: vec![1] }]),
            console(&[Frame::Signal { signal: 11 }]),
            console(&[Frame::Failed {
                reason: "pilot refused execution".into(),
            }]),
        ] {
            assert!(!parse_report(&text, "cell".into(), 10, false, true).succeeded());
        }
        let exit = console(&[Frame::Exit { code: 0 }]);
        assert!(!parse_report(&exit, "cell".into(), 10, true, true).succeeded());
        assert!(!parse_report(&exit, "cell".into(), 10, false, false).succeeded());
        let old_pilot =
            format!("CELLN:pilot=alive\nCELLN:pilot_manifest=signed\nCELLN:out-begin\n{exit}");
        assert!(!parse_report(&old_pilot, "cell".into(), 10, false, true).succeeded());
    }

    // The kernel owns the console until pilot silences it, and emergency
    // messages (lockups, the reboot notice) bypass that: a kernel line may land
    // on the same console line as a marker or a frame. Such text cannot forge
    // a frame, so it is tolerated wherever a frame has already been read whole.
    #[test]
    fn kernel_text_around_markers_and_after_frames_is_tolerated() {
        let exit = serde_json::to_string(&Frame::Exit { code: 0 }).unwrap();
        let output = serde_json::to_string(&Frame::Output {
            bytes: b"answer".to_vec(),
        })
        .unwrap();
        let console = format!(
            "d\nCELLN:cell=live\n[    0.000000] Linux version 6.1\nCELLN:pilot=alive[    0.52] random: crng init done\n\
             [    0.61] clocksource: Switched to clocksource tsc\n{PROTOCOL}[    0.70] EXT4-fs: mounted\n\
             CELLN:pilot_manifest=signed\n{PREFIX}{output}\n{PREFIX}{exit}[   35.47] watchdog: BUG: soft lockup - CPU#0 stuck\n\
             [   35.50] reboot: Restarting system\n"
        );
        let result = parse_report(&console, "cell".into(), 1024, false, true);
        assert!(result.succeeded(), "{:?}", result.denial);
        assert_eq!(result.output.unwrap(), b"answer");
        // Nothing after a frame may carry another frame, and the markers must
        // end where they end: a longer protocol number is another protocol.
        let started = serde_json::to_string(&Frame::Started {
            grant: pilot::dispatch_report::ExecutionGrant {
                tool: celln_manifest::Hash::of(b"program").0,
                lane: "agent".into(),
                workspace: Some("none".into()),
                fetch: false,
            },
        })
        .unwrap();
        for text in [
            format!("CELLN:pilot=alive\n{PROTOCOL}\n{PREFIX}{exit}{started}\n"),
            format!("CELLN:pilot=alive\n{PROTOCOL}\n{PREFIX}{exit} {PREFIX}{started}\n"),
            format!("CELLN:pilot=alive\n{PROTOCOL}0\n{PREFIX}{exit}\n"),
            format!("CELLN:pilot=alive2\n{PROTOCOL}\n{PREFIX}{exit}\n"),
        ] {
            let result = parse_report(&text, "cell".into(), 1024, false, true);
            assert!(!result.succeeded(), "{text:?}");
            assert!(result.execution.is_none());
        }
    }

    // A rejected report says which check failed, bounded, so an operator can
    // tell a kernel interruption from a lost exit from an unclean guest end.
    #[test]
    fn rejected_reports_name_their_defect() {
        let exit = console(&[Frame::Exit { code: 0 }]);
        let denial = |text: &str, shutdown: bool| {
            parse_report(text, "cell".into(), 10, false, shutdown)
                .denial
                .unwrap_or_default()
        };
        for (text, shutdown, cause) in [
            (format!("{PREFIX}broken\n"), true, "negotiation"),
            (
                format!("CELLN:pilot=alive\n{PROTOCOL}\n{PREFIX}{{\"kind\":\"exit\",\"co[  3.1] printk\nCELLN:done\n"),
                true,
                "unreadable dispatch frame",
            ),
            (
                format!("CELLN:pilot=alive\n{PROTOCOL}\nCELLN:out-end\n"),
                true,
                "no terminal frame",
            ),
            (exit.clone(), false, "clean shutdown"),
            (
                console(&[Frame::Exit { code: 0 }, Frame::Output { bytes: vec![1] }]),
                true,
                "after the terminal frame",
            ),
            (
                console(&[Frame::Exit { code: 300 }]),
                true,
                "exit frame out of order or out of range",
            ),
        ] {
            let denial = denial(&text, shutdown);
            assert!(denial.starts_with(REPORT_DENIAL), "{denial}");
            assert!(denial.contains(cause), "{denial} should name {cause}");
        }
        let long = format!(
            "CELLN:pilot=alive\n{PROTOCOL}\n{PREFIX}{}\n",
            "x".repeat(1000)
        );
        let denial = denial(&long, true);
        assert!(denial.chars().count() < 400, "{}", denial.len());
        assert!(denial.ends_with('…'));
    }
}
