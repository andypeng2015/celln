//! Operator timing traces, off unless `CELLN_TRACE=1` is set on the
//! dispatcher. Lines name a phase and its duration only: no URLs, bodies,
//! credentials or guest text, so they are safe to leave enabled on a node
//! while its latency is being studied.
use std::sync::OnceLock;
use std::time::Instant;

pub fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var("CELLN_TRACE").as_deref() == Ok("1"))
}

/// Prints `celln trace: <what> <ms> ms` when enabled.
pub fn span(what: &str, since: Instant) {
    if enabled() {
        eprintln!("celln trace: {what} {} ms", since.elapsed().as_millis());
    }
}

/// Prints `celln trace: <what>` with a caller-formatted detail when enabled.
pub fn note(what: &str, detail: &str) {
    if enabled() {
        eprintln!("celln trace: {what} {detail}");
    }
}
