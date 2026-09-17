# Changelog

## [0.5.21](https://github.com/andypeng2015/celln/compare/v0.5.20...v0.5.21) (2026-09-17)


### Features

* `nous` — the CLI, and a spec users actually write ([2dbb38f](https://github.com/andypeng2015/celln/commit/2dbb38fe4b44712355752394628a07a3dc33fd35))
* add agent setup and runtime plans ([dd9685f](https://github.com/andypeng2015/celln/commit/dd9685f7bfada6785e4b882771f9ebcd324a9acb))
* add DeepSeek agent backend and default root to $HOME/.celln ([954c76d](https://github.com/andypeng2015/celln/commit/954c76d308ffbc4ee8ec46fd2cb6612d8b3989fb))
* authenticate and dispatch sealed dependency closures ([44ab492](https://github.com/andypeng2015/celln/commit/44ab492accf0819b7b34073590a87a3ba48f8dee))
* auto-detect agent backends during install ([b6c6c62](https://github.com/andypeng2015/celln/commit/b6c6c6211d27a601bc803c621ed26dfe67443f2e))
* automated installer DaemonSet ([0aa6761](https://github.com/andypeng2015/celln/commit/0aa676165a24fdb49a8c926853c7c4bf3e286420))
* broker bounded HTTP fetch through pilot ([15a620f](https://github.com/andypeng2015/celln/commit/15a620fa7b3aa1fb2c81d8b0a019a69370999231))
* brokered egress from a spec cell (allow_hosts) ([0c79633](https://github.com/andypeng2015/celln/commit/0c79633e5306b285283ab17fd498914f112e03bd))
* **cli:** celln image add — one command to put a tool in the catalogue ([3a3d4cc](https://github.com/andypeng2015/celln/commit/3a3d4ccdc2da77563ea1649908244ac3eb035ba4))
* **cli:** celln image pull — materialise OCI images as sealed filesystems ([b89a86b](https://github.com/andypeng2015/celln/commit/b89a86b6d7055b3c0cc21bd36d0ffcbabee0058b))
* **cli:** celln image spec — scaffold a runnable spec from the catalogue ([58b6f98](https://github.com/andypeng2015/celln/commit/58b6f9876cd0d3d2d55e58283bfc61145f56c097))
* **cli:** ship a pinned tool catalogue and pull it at setup ([bb32953](https://github.com/andypeng2015/celln/commit/bb329535a408eca46bb3a68b6f3840919a4d3aec))
* **cli:** standalone operator-signed native starter packaging ([#99](https://github.com/andypeng2015/celln/issues/99)) ([e1385eb](https://github.com/andypeng2015/celln/commit/e1385eb81916a955ffdf578f9b4d8dd82910be20))
* **closure:** add bound read-only verification reports and shared policy checks ([19dad57](https://github.com/andypeng2015/celln/commit/19dad5768173f61a5c713f5731846e10e9152a3c))
* **closure:** bound read-only verification and shared policy checks ([f789a35](https://github.com/andypeng2015/celln/commit/f789a35b035d87c5862371558f1247a9e3640b0c))
* **closure:** sealed guest member verification without tool execution ([f051812](https://github.com/andypeng2015/celln/commit/f05181207c6ba972cf20d2ce63bc2e30eca1883e))
* **closure:** verify bounded local filesystem bytes for review ([a71147c](https://github.com/andypeng2015/celln/commit/a71147c06bee5e59b69272f2f1b741629200120e))
* **closure:** verify local artifact bytes for catalogue review ([750053c](https://github.com/andypeng2015/celln/commit/750053cb7fb8780366b9548f300d94ab49ecbf59))
* **closure:** verify member identities inside a sealed cell without exec ([f4a470b](https://github.com/andypeng2015/celln/commit/f4a470ba26fa701fcf3f165b973bc568b8d65ff4))
* **dispatch:** cancel executions under one end-to-end deadline ([fccb701](https://github.com/andypeng2015/celln/commit/fccb701f26707adfcdd8392a4db970c7fb3f7f11))
* **dispatch:** cancellation and end-to-end deadlines ([64174a7](https://github.com/andypeng2015/celln/commit/64174a7e118d4255f28e9c6158bc2a16934b3ec5))
* **egress:** opt-in HTTP and self-signed model endpoints ([#101](https://github.com/andypeng2015/celln/issues/101)) ([ccc6367](https://github.com/andypeng2015/celln/commit/ccc63677643653e0687b139a5c48188df2802c1c))
* **egress:** prototype scoped model POST broker with real guest proof ([60ffd40](https://github.com/andypeng2015/celln/commit/60ffd40da3805eed260d472941d93a61ba4e5f83))
* enforce boot-bound model profile admission expiry ([#93](https://github.com/andypeng2015/celln/issues/93)) ([064b568](https://github.com/andypeng2015/celln/commit/064b56814247f6d13997ad2d82c4e6129dd52312))
* forge — the Forged tier is now earned ([5bb746d](https://github.com/andypeng2015/celln/commit/5bb746df1d2efad80beb6d46c7df315319d007cf))
* forge-from-task in /v1/executions, remove /v1/actions ([3c16cc6](https://github.com/andypeng2015/celln/commit/3c16cc68ec73fa01c355b9c13a4f81c14c9728cf))
* **harness:** 32 KiB model request wire budget for two dozen tools ([0063841](https://github.com/andypeng2015/celln/commit/0063841dfcaf6e526213bc5309bb7ae6dce4c5a3))
* **harness:** authorize versioned JSON tool dispatch with pinned schemas ([f4b99a5](https://github.com/andypeng2015/celln/commit/f4b99a5ea65fc9d3c2c2357e7aa6335a447071ef))
* **harness:** bind approved runtime tools and model grants in dispatch ([d92651e](https://github.com/andypeng2015/celln/commit/d92651e0e2f90ecafcad40c224a45312b97902b4))
* **harness:** bounded schema-bound JSON tool adapter with real-model proof ([33b6f2d](https://github.com/andypeng2015/celln/commit/33b6f2d2a5a69a076bf26314a1259520bc58dae1))
* **harness:** dispatch approved in-cell runtime/tool/model bindings ([264bc06](https://github.com/andypeng2015/celln/commit/264bc06d39bcae4edea05cc09aa9d1b6b1502c19))
* **harness:** document requirements and prove in-cell model POST (prototype) ([96c47c3](https://github.com/andypeng2015/celln/commit/96c47c3133b97d109a5673938e681a31f61e38ec))
* **harness:** enforce host model grants and output reservations ([cff8067](https://github.com/andypeng2015/celln/commit/cff8067b8d196ff12b8615e13ac5269b98160673))
* **harness:** enforce model grants and prove guest budget denials ([953639e](https://github.com/andypeng2015/celln/commit/953639edcf84213e28144a9b362ff24c440588d0))
* **harness:** host-authorized JSON tool dispatch and real-model proof ([f55c3a2](https://github.com/andypeng2015/celln/commit/f55c3a2270574ed61828d5f03e77a6d25b609aee))
* **harness:** prove in-cell model loop with separately lent tools ([da6e1ff](https://github.com/andypeng2015/celln/commit/da6e1ff7a65f8c2f41663881a83d51a86e172e26))
* **harness:** prove warm-forked in-cell model loop with lent tools ([2f3ab80](https://github.com/andypeng2015/celln/commit/2f3ab807c939d9e4db5fa3155f50cb826bee3635))
* **harness:** schema-bound JSON borrowed tools and real-model proof ([7d9116f](https://github.com/andypeng2015/celln/commit/7d9116f30f5fa6be428a7e859491af2fef3ebd4c))
* hermetic boundary demo — four breakout attempts, one legitimate computation ([af8e0ca](https://github.com/andypeng2015/celln/commit/af8e0ca9bd596d2f2a62a13892db9e8564963613))
* host-level dispatcher + router + Sympozium integration ([f09b197](https://github.com/andypeng2015/celln/commit/f09b1977e203eb7b5e1fd3d406f977c98efcd071))
* host-level dispatcher, router, Sympozium integration ([9e10444](https://github.com/andypeng2015/celln/commit/9e10444c0b0944a77e6c0c2ea1368a3593b9350f))
* interactive backend selection on celln setup ([bbfbc5a](https://github.com/andypeng2015/celln/commit/bbfbc5a875dcb95e16a2ef5e8360a6de162030e4))
* issue request-bound Harness grants from live host policy ([#91](https://github.com/andypeng2015/celln/issues/91)) ([f266cb0](https://github.com/andypeng2015/celln/commit/f266cb027cc12e94a237c8bea0b314bb15aca211))
* **k8s:** wire ANTHROPIC_API_KEY and OPENAI_API_KEY secrets to installer ([0ad7b55](https://github.com/andypeng2015/celln/commit/0ad7b55ce9481246b363b6b26ee500a8f6b208a2))
* let a model write what a cell runs ([de25dbc](https://github.com/andypeng2015/celln/commit/de25dbc1abb1ac298164243d5e5c3fa38359809d))
* mount several sealed images into one cell ([a5eb22d](https://github.com/andypeng2015/celln/commit/a5eb22dce4b4c7992db516a4e073362a68b9df4d))
* native persistent parent with disposable starter-tool cells ([#98](https://github.com/andypeng2015/celln/issues/98)) ([a497b27](https://github.com/andypeng2015/celln/commit/a497b275114cdb052a85d02e3e371aadc52a3943))
* nous agent — a model writes code, a cell runs it ([e3b8b96](https://github.com/andypeng2015/celln/commit/e3b8b96d279f01505a0c309eb34308711036a393))
* nous ps — see cells, the way docker ps shows containers ([9aaa745](https://github.com/andypeng2015/celln/commit/9aaa74519a4dca830772d27802537dbba1da6a3b))
* **parent:** seed a new parent with the conversation it continues ([da72b36](https://github.com/andypeng2015/celln/commit/da72b361753bbb45b7070095ff654933932d4c80))
* **parent:** seed a new parent with the conversation it continues ([a63050c](https://github.com/andypeng2015/celln/commit/a63050c1c92408b835a2b5f8904baf9d1f711268))
* **parents:** provision on the owning dispatcher; router binds affinity at provisioning ([#109](https://github.com/andypeng2015/celln/issues/109)) ([bd17846](https://github.com/andypeng2015/celln/commit/bd1784618ba011cddba4f559f80fb45437949539))
* per-backend credential validation on celln setup ([09cfd60](https://github.com/andypeng2015/celln/commit/09cfd609e3735f44146c08d3e8da880834972800))
* **pilot:** chroot into a sealed image so a tool closure can run ([a734d9c](https://github.com/andypeng2015/celln/commit/a734d9c2a4c1006d431db2d76605a9e848237657))
* **pilot:** pilot runs inside the cell and enforces exec-by-hash ([68644fd](https://github.com/andypeng2015/celln/commit/68644fd6fdee3ebbd2615212873c586146593f5c))
* pin router prewarm and execution to configured hosts ([#94](https://github.com/andypeng2015/celln/issues/94)) ([7151d3e](https://github.com/andypeng2015/celln/commit/7151d3e90473e39433272576268cba550a8c2669))
* prepare candidate motes from operator-pinned templates ([#95](https://github.com/andypeng2015/celln/issues/95)) ([dd75273](https://github.com/andypeng2015/celln/commit/dd75273b4a0c0012273b36137a62055a69357ae4))
* remove celln ask; lead the README with a two-tool cell ([ac15edd](https://github.com/andypeng2015/celln/commit/ac15edd5be958c2b98ba555ac897865e028b0688))
* router DaemonSet with downward API host IP discovery ([1ea92e5](https://github.com/andypeng2015/celln/commit/1ea92e5c7e1e0cea4949320e6e7f888ef069c24d))
* **router:** add scoped authenticated capability discovery ([5a330cd](https://github.com/andypeng2015/celln/commit/5a330cdf4b017d9643a9834f6ff3823fdfc2c845))
* **router:** authenticated read-only capability discovery ([b5f1402](https://github.com/andypeng2015/celln/commit/b5f14028fd49a24812c15d981afb0efe6bbe7eae))
* **router:** place new parents on the owner with the most spare capacity ([8d515bd](https://github.com/andypeng2015/celln/commit/8d515bd8cfdcc665a4fdfabbcfe1fc8a3ab76d40))
* **router:** place new parents on the owner with the most spare capacity ([38e5d14](https://github.com/andypeng2015/celln/commit/38e5d147144db3640cad9639cd02f0e52cea0b44))
* **router:** re-resolve --backends-srv so owners can join or leave live ([#110](https://github.com/andypeng2015/celln/issues/110)) ([4c08bcd](https://github.com/andypeng2015/celln/commit/4c08bcdba1e50df8d0227a8925651367445592a0))
* **router:** route /v1/parents with durable parent affinity; dispatcher drain ([#103](https://github.com/andypeng2015/celln/issues/103)) ([f5403e5](https://github.com/andypeng2015/celln/commit/f5403e507748985de740febd2e1b98c65461ae97))
* run agent code in bounded agent lane ([8b7169d](https://github.com/andypeng2015/celln/commit/8b7169d9f1398f0e939bf147e7d5d550e327042e))
* run any tool in a cell, by lending its dependency closure ([acf2819](https://github.com/andypeng2015/celln/commit/acf2819cf4892ea3276208bd6e06925578c1313f))
* run generated programs in the safe agent cell lane ([ab1ad66](https://github.com/andypeng2015/celln/commit/ab1ad6694609594646dcd02396fcaa1ace4f68d7))
* run several tools in one cell (M5) ([b14d6a6](https://github.com/andypeng2015/celln/commit/b14d6a65eb216dff40e302012962eb5f416e7b51))
* **schema:** bounded immutable tool schema and data verification ([9a4938a](https://github.com/andypeng2015/celln/commit/9a4938a858e83d9b4207b46f8cf05eaa226ac7ab))
* **schema:** verify immutable bounded tool schemas and JSON data ([60ebe81](https://github.com/andypeng2015/celln/commit/60ebe819fad1505c6b1ae861045d3c3b50fd3248))
* signed runtime/tool closure composition ([#88](https://github.com/andypeng2015/celln/issues/88)) ([1f294a5](https://github.com/andypeng2015/celln/commit/1f294a5a3f9a0cc7a8ff22db6693bafb144dc162))
* **spec:** lend a tool as an image, not just a file ([9e19c54](https://github.com/andypeng2015/celln/commit/9e19c540d62b52a5567b4f67c80b64b2056dd0a2))
* **starter:** operator host limits — lease up to 24h, turn and token ceilings per plan ([c922759](https://github.com/andypeng2015/celln/commit/c922759c9923e930f9c950112cad6232e4c90ea6))
* **starter:** operator host limits — lease up to 24h, turn and token ceilings per plan ([74817f1](https://github.com/andypeng2015/celln/commit/74817f1d32852c261691883cc0eb43162496ef59))
* **starter:** workspace list/append/search/delete and credential-free JSON POST tools ([4dc50b9](https://github.com/andypeng2015/celln/commit/4dc50b91974f6f042d1fa5f9b929bd31d83c30c9))
* **starter:** workspace list/append/search/delete and credential-free JSON POST tools ([cad57b3](https://github.com/andypeng2015/celln/commit/cad57b363d10ecdd587067016597c6a3cf2706cf))
* **tools:** borrow real commands from pinned images as argv tools ([b16db8a](https://github.com/andypeng2015/celln/commit/b16db8ac628db33c339ecff4464f0b4d5fdf440d))
* **tools:** borrow real commands from pinned images as argv tools ([4c90df5](https://github.com/andypeng2015/celln/commit/4c90df5268d23db391f32859c6795738b1992e1b))
* trusted precomposed closure dispatch and regression proof ([b43133f](https://github.com/andypeng2015/celln/commit/b43133f8550e581b37c6caf6679a0e89c96483dc))
* verify and prewarm admitted artifacts in serving dispatcher ([#90](https://github.com/andypeng2015/celln/issues/90)) ([9444346](https://github.com/andypeng2015/celln/commit/94443467eb2e87caf42fbb8ac86d24263b7c82d1))
* **warden:** size the tool window to the image (F1) ([fc64453](https://github.com/andypeng2015/celln/commit/fc64453553cf52a77a2cf6045a45a007e72ccd63))
* wire the dispatcher to the ExecutionRequest/receipt contract ([19be193](https://github.com/andypeng2015/celln/commit/19be193f9ae3ab918fca2eae985fa6e43d767533))


### Bug Fixes

* agent-authored code never reaches the tool lane; pick your backend ([0cf3e76](https://github.com/andypeng2015/celln/commit/0cf3e76725e1affe651c503530df68154878a65b))
* allow brokered fetches from agent cells ([afc1f50](https://github.com/andypeng2015/celln/commit/afc1f5080f2e39556595db0ff5991cea09e4fa95))
* **assay:** a declared interpreter must beat a stored entry that was not ([68f9928](https://github.com/andypeng2015/celln/commit/68f9928413a266d1a7d4500ab95af0cd58f7fa6a))
* **assay:** a warm hit must match the bytes, not just the alias ([1267b95](https://github.com/andypeng2015/celln/commit/1267b9553edda042ef17ab126302519d6678771f))
* **assay:** preserve verified artifact authorship ([#45](https://github.com/andypeng2015/celln/issues/45)) ([50bfb36](https://github.com/andypeng2015/celln/commit/50bfb3625602380eb26fcc4a9193f618f31ddfb7))
* bind the sidebar nav's CSS class on every docs page ([03f48c0](https://github.com/andypeng2015/celln/commit/03f48c0f159b7b95e1f028adf4fd736d1b1ee2f7))
* **cli:** enforce node cell capacity at admission ([#46](https://github.com/andypeng2015/celln/issues/46)) ([644cb43](https://github.com/andypeng2015/celln/commit/644cb439b065ec4fb22c819420c1e79b960f6524))
* close top 3 P0 findings from the critical-review epic ([#29](https://github.com/andypeng2015/celln/issues/29)) ([#44](https://github.com/andypeng2015/celln/issues/44)) ([e3f5057](https://github.com/andypeng2015/celln/commit/e3f5057044fa6bb7fdb0d78a4366d6c6944bd4ab))
* collect the diagnosis instead of waiting out a silent hang ([7a6a58e](https://github.com/andypeng2015/celln/commit/7a6a58ea21de2520802c54f7d8ca2cc8a613c2c3))
* create /etc/celln dir before writing agent-key file ([632d507](https://github.com/andypeng2015/celln/commit/632d507f42e0fd7787cf7471c7adca2f4b2f43f2))
* **dispatch:** derive results from guest exit status ([#56](https://github.com/andypeng2015/celln/issues/56)) ([b53be10](https://github.com/andypeng2015/celln/commit/b53be10f8cb41bac41bf55706b95cef6bcb9ac79))
* **dispatcher:** bound model requests by the turn deadline instead of a fixed 45 seconds ([24b30c0](https://github.com/andypeng2015/celln/commit/24b30c000a363dc9d227c9f074d0a76dad9eb024))
* **dispatcher:** bound model requests by the turn deadline instead of a fixed 45 seconds ([7be438c](https://github.com/andypeng2015/celln/commit/7be438c08fee16805cfbb1d1a4f4c101db612591))
* **dispatcher:** charge exact broker slots per parent so a node holds more than one parent ([a1aa888](https://github.com/andypeng2015/celln/commit/a1aa88808ad9970f7a48c2e5150ed59a0e763c00))
* **dispatcher:** charge exact broker slots per parent so a node holds more than one parent ([2341461](https://github.com/andypeng2015/celln/commit/2341461a3d06b06d38aaca8a71f7fb36a2bca3f2))
* **dispatcher:** enforce transport and egress policy ([#48](https://github.com/andypeng2015/celln/issues/48)) ([bb14b6e](https://github.com/andypeng2015/celln/commit/bb14b6e0d354b91d7e4f74ddf0a18fe232a73156))
* **dispatcher:** persist admission tombstones and terminal execution records ([463356c](https://github.com/andypeng2015/celln/commit/463356c6fe5b0c8efa3a643dd400f1161fbbcc45))
* **dispatcher:** persist execution claims receipts and audits across restart ([e8fb6a5](https://github.com/andypeng2015/celln/commit/e8fb6a5ce9b4769d865056ab0caa382779eb3d9e))
* **dispatcher:** reload bounded credentials per protected request ([a0a4938](https://github.com/andypeng2015/celln/commit/a0a49386cb168dc3423033ae5d60ae550596bb53))
* **dispatcher:** reload credentials safely without restart ([585ced1](https://github.com/andypeng2015/celln/commit/585ced1e26ea32f76cce30969ef93412bd889c76))
* **dispatch:** execute operator-pinned declared substrates ([45c1932](https://github.com/andypeng2015/celln/commit/45c193274957b9959d4d1588c57b76b79e5ba2a7))
* **dispatch:** execute operator-pinned declared substrates ([9a34b32](https://github.com/andypeng2015/celln/commit/9a34b32d35a4856e6349248a2c46615b5f3a963a))
* **dispatch:** fork executions from prepared warm motes ([bef1ee6](https://github.com/andypeng2015/celln/commit/bef1ee69b39899a30651a73743ed359e0b486a3d))
* **dispatch:** fork executions from prepared warm motes ([ad34fc0](https://github.com/andypeng2015/celln/commit/ad34fc0318c73f9d41aa2e5122fdacb4790e2c0b))
* **dispatch:** refuse unsupported input and closure authority ([b0a31b9](https://github.com/andypeng2015/celln/commit/b0a31b9d38a917856b8f640f05240696a00bed76))
* **dispatch:** refuse unsupported input and closure authority ([b410ec7](https://github.com/andypeng2015/celln/commit/b410ec774593f839bbaa90ef1785b875ce1742ec))
* **egress:** accept Anthropic thinking blocks without forwarding them ([bbfab46](https://github.com/andypeng2015/celln/commit/bbfab4653253a0a55985d19da8c462bad1924de6))
* **egress:** accept Anthropic thinking blocks without forwarding them ([a8c8682](https://github.com/andypeng2015/celln/commit/a8c8682d929bbbdd28220242125340836a840d02))
* forge in-process, and stop hanging without a deadline ([906b235](https://github.com/andypeng2015/celln/commit/906b2358e16d7a6d5c79b2231cb90acaf37d4e52))
* give a node its tool images, and size them so they build ([#21](https://github.com/andypeng2015/celln/issues/21)) ([71c7864](https://github.com/andypeng2015/celln/commit/71c786497154cdba8aa66b3ee61773760ab172dc))
* **guest:** always remount a sealed image read-only (F2) ([122bbd0](https://github.com/andypeng2015/celln/commit/122bbd09454778758bd8fb7c776cbfd972041115))
* **harness:** refuse tool side effects without a remaining result turn ([fbc772e](https://github.com/andypeng2015/celln/commit/fbc772e03909416130f13b8c4320ce4deb5a1b83))
* install musl target before release verification ([4f05238](https://github.com/andypeng2015/celln/commit/4f052385c30e63aa7c4b09e671ce2f985e8f45c8))
* install release archives from Homebrew ([ff01ad9](https://github.com/andypeng2015/celln/commit/ff01ad9d66203cb8dfa001c283bbf25ce7f65b3e))
* make it impossible to publish crates in a broken order ([#23](https://github.com/andypeng2015/celln/issues/23)) ([11dbf17](https://github.com/andypeng2015/celln/commit/11dbf1791dc367fd7d93b84135125b59a22f8009))
* make release builds reproducible ([fabff05](https://github.com/andypeng2015/celln/commit/fabff05e60a354a901a0122c08a846a137c9e836))
* **manifest:** resolve_alias must skip revoked entries; list tools ([36961f6](https://github.com/andypeng2015/celln/commit/36961f6c3c448487c82cd80666d2ea6b8ad04cb3))
* parameterize setup-k8s.sh instead of hardcoding axjns paths ([eaa9edd](https://github.com/andypeng2015/celln/commit/eaa9edd036878875dd64ec5f06f40f6bc9174cc7))
* **parent:** a failed child is a failed turn, not lost parent context ([5ebbcfb](https://github.com/andypeng2015/celln/commit/5ebbcfb8291dba0a4760fafa54912c73e44d6966))
* **parent:** a failed child is a failed turn, not lost parent context ([6d04a99](https://github.com/andypeng2015/celln/commit/6d04a9945e51c0c97c8f4b171799d17689365759)), closes [#124](https://github.com/andypeng2015/celln/issues/124)
* **pilot:** exec the exact verified file descriptor ([#49](https://github.com/andypeng2015/celln/issues/49)) ([c7c9730](https://github.com/andypeng2015/celln/commit/c7c97302b572f9dc4047581205e499301ea484cd))
* **pilot:** narrow agent-lane Linux capabilities ([#54](https://github.com/andypeng2015/celln/issues/54)) ([fe99c0e](https://github.com/andypeng2015/celln/commit/fe99c0e02c4ad1b17b0003d5016af9ec3f25d069))
* provider-agnostic key setup (OpenAI, Anthropic, DeepSeek, ollama) ([05aef08](https://github.com/andypeng2015/celln/commit/05aef082d6d67c6f12bc12ea2ec7cae3905a0f28))
* publish the Celln formula on release ([bb7543f](https://github.com/andypeng2015/celln/commit/bb7543f6b3ff5ec22b6a892a34c35d87734ba1b2))
* record and explain refused agent cells ([2ffb620](https://github.com/andypeng2015/celln/commit/2ffb62042cf96b22a34c8974c2a49153cce93716))
* rename cell 'name' to 'description', truncate to 30 chars ([03506c2](https://github.com/andypeng2015/celln/commit/03506c2230f43ae52d576ff3c0fee3afe393ae3d))
* repair the acceptance script, and correct stale docs ([32ca5a3](https://github.com/andypeng2015/celln/commit/32ca5a372fcd906c5e9434d047c91cc2cdb701e7))
* restore the VFS&lt;-&gt;memslot proofs broken by the Celln rename ([339410f](https://github.com/andypeng2015/celln/commit/339410f4c074f3435dfc7311ef8ed3f4b7c59591))
* router never proxied /v1/executions, only /v1/actions ([9b308b2](https://github.com/andypeng2015/celln/commit/9b308b2ad3846560ea27a0b28c103a95f85ac7a2))
* **router:** authenticate callers and forward cancellation (M0) ([683bcb9](https://github.com/andypeng2015/celln/commit/683bcb93eb4f18f2cb64415e098763ea2de7312a))
* **router:** deliver bounded early refusals without TCP resets ([#97](https://github.com/andypeng2015/celln/issues/97)) ([c83be71](https://github.com/andypeng2015/celln/commit/c83be719872c97983c1cba307246503425a5a193))
* **router:** persist ownership before dispatch and refuse ambiguous replay ([577699d](https://github.com/andypeng2015/celln/commit/577699d1da286344f671325c2f28725ee0fdaaab))
* **router:** require distinct client auth and forward cancellation ([7ea49f3](https://github.com/andypeng2015/celln/commit/7ea49f3f986a6894b040e5bd22f669d716649dca))
* **router:** retain execution ownership and refuse ambiguous replay ([0b99751](https://github.com/andypeng2015/celln/commit/0b997518a9f46f0e26b426b28122eaef4cd280ab))
* satisfy clippy -D warnings on the health-check auth bypass check ([7c63530](https://github.com/andypeng2015/celln/commit/7c635303314818406d231a7c91d8d36fdfc2dc86))
* say what `nous agent` is actually doing ([729727a](https://github.com/andypeng2015/celln/commit/729727ab5fb5241f427adc9985ec3b4cee687382))
* scope pilot fetch I/O permissions ([#47](https://github.com/andypeng2015/celln/issues/47)) ([a483243](https://github.com/andypeng2015/celln/commit/a4832435e347c1ca9801159bc478680a642a5466))
* self-bootstrapping runtime assets and descriptive cell names ([ee295a1](https://github.com/andypeng2015/celln/commit/ee295a1b1bc8b0d4aa5e92e137a442120d300168))
* ship a static guest pilot in Homebrew ([8d758a1](https://github.com/andypeng2015/celln/commit/8d758a1c550b3a11da4bff12ff15cbd2666ecc10))
* show tool images by name, not by digest ([#20](https://github.com/andypeng2015/celln/issues/20)) ([8c7866d](https://github.com/andypeng2015/celln/commit/8c7866d2c17701d38f7c8f6c6cc7c94e6ddbe8f4))
* **starter:** look for packaging tools under sbin too ([fa70d12](https://github.com/andypeng2015/celln/commit/fa70d12d27b56a148ef2c215fe60e568008a31c5))
* **starter:** look for packaging tools under sbin too ([3ca6b80](https://github.com/andypeng2015/celln/commit/3ca6b806a8228bd4d2cdffd0bb3e3ad512ad5435))
* tell people how to add a tool, and make both that and the digest refresh work ([#22](https://github.com/andypeng2015/celln/issues/22)) ([48bd77d](https://github.com/andypeng2015/celln/commit/48bd77d17341ce87e8095100e015158f468cdf42))
* tell people how to add a tool, and make both that and the digest refresh work ([#22](https://github.com/andypeng2015/celln/issues/22)) ([ddadcf7](https://github.com/andypeng2015/celln/commit/ddadcf73a953ec9eaf14a9f5eb644351e15da63b))
* update aligned formula tags on release ([43dad92](https://github.com/andypeng2015/celln/commit/43dad9251f2f86103c6f31f8d3f515eb94feb554))
* use the renamed assay package in initramfs staging ([3fc732e](https://github.com/andypeng2015/celln/commit/3fc732e46d922222dd55cbb868f66c3eaffe1e2f))
* **vmm:** buffer 32 KiB broker requests from a cell ([1ba40eb](https://github.com/andypeng2015/celln/commit/1ba40ebf488c766cc6aee170c66452fc2b594a85))
* write provider config with private mode ([#50](https://github.com/andypeng2015/celln/issues/50)) ([16ea93c](https://github.com/andypeng2015/celln/commit/16ea93c1f0fc01ce84e5ab77bf92fdd0feb64cdd))

## [0.5.20](https://github.com/sympozium-ai/celln/compare/v0.5.19...v0.5.20) (2026-09-15)


### Bug Fixes

* **starter:** look for packaging tools under sbin too ([fa70d12](https://github.com/sympozium-ai/celln/commit/fa70d12d27b56a148ef2c215fe60e568008a31c5))
* **starter:** look for packaging tools under sbin too ([3ca6b80](https://github.com/sympozium-ai/celln/commit/3ca6b806a8228bd4d2cdffd0bb3e3ad512ad5435))

## [0.5.19](https://github.com/sympozium-ai/celln/compare/v0.5.18...v0.5.19) (2026-09-15)


### Features

* **harness:** 32 KiB model request wire budget for two dozen tools ([0063841](https://github.com/sympozium-ai/celln/commit/0063841dfcaf6e526213bc5309bb7ae6dce4c5a3))
* **starter:** workspace list/append/search/delete and credential-free JSON POST tools ([4dc50b9](https://github.com/sympozium-ai/celln/commit/4dc50b91974f6f042d1fa5f9b929bd31d83c30c9))
* **starter:** workspace list/append/search/delete and credential-free JSON POST tools ([cad57b3](https://github.com/sympozium-ai/celln/commit/cad57b363d10ecdd587067016597c6a3cf2706cf))


### Bug Fixes

* **vmm:** buffer 32 KiB broker requests from a cell ([1ba40eb](https://github.com/sympozium-ai/celln/commit/1ba40ebf488c766cc6aee170c66452fc2b594a85))

## [0.5.18](https://github.com/sympozium-ai/celln/compare/v0.5.17...v0.5.18) (2026-09-15)


### Features

* **tools:** borrow real commands from pinned images as argv tools ([b16db8a](https://github.com/sympozium-ai/celln/commit/b16db8ac628db33c339ecff4464f0b4d5fdf440d))
* **tools:** borrow real commands from pinned images as argv tools ([4c90df5](https://github.com/sympozium-ai/celln/commit/4c90df5268d23db391f32859c6795738b1992e1b))

## [0.5.17](https://github.com/sympozium-ai/celln/compare/v0.5.16...v0.5.17) (2026-09-14)


### Bug Fixes

* **parent:** a failed child is a failed turn, not lost parent context ([5ebbcfb](https://github.com/sympozium-ai/celln/commit/5ebbcfb8291dba0a4760fafa54912c73e44d6966))
* **parent:** a failed child is a failed turn, not lost parent context ([6d04a99](https://github.com/sympozium-ai/celln/commit/6d04a9945e51c0c97c8f4b171799d17689365759)), closes [#124](https://github.com/sympozium-ai/celln/issues/124)

## [0.5.16](https://github.com/sympozium-ai/celln/compare/v0.5.15...v0.5.16) (2026-09-14)


### Features

* **router:** place new parents on the owner with the most spare capacity ([8d515bd](https://github.com/sympozium-ai/celln/commit/8d515bd8cfdcc665a4fdfabbcfe1fc8a3ab76d40))
* **router:** place new parents on the owner with the most spare capacity ([38e5d14](https://github.com/sympozium-ai/celln/commit/38e5d147144db3640cad9639cd02f0e52cea0b44))

## [0.5.15](https://github.com/sympozium-ai/celln/compare/v0.5.14...v0.5.15) (2026-09-14)


### Features

* **starter:** operator host limits — lease up to 24h, turn and token ceilings per plan ([c922759](https://github.com/sympozium-ai/celln/commit/c922759c9923e930f9c950112cad6232e4c90ea6))
* **starter:** operator host limits — lease up to 24h, turn and token ceilings per plan ([74817f1](https://github.com/sympozium-ai/celln/commit/74817f1d32852c261691883cc0eb43162496ef59))

## [0.5.14](https://github.com/sympozium-ai/celln/compare/v0.5.13...v0.5.14) (2026-09-14)


### Bug Fixes

* **egress:** accept Anthropic thinking blocks without forwarding them ([bbfab46](https://github.com/sympozium-ai/celln/commit/bbfab4653253a0a55985d19da8c462bad1924de6))
* **egress:** accept Anthropic thinking blocks without forwarding them ([a8c8682](https://github.com/sympozium-ai/celln/commit/a8c8682d929bbbdd28220242125340836a840d02))

## [0.5.13](https://github.com/sympozium-ai/celln/compare/v0.5.12...v0.5.13) (2026-09-14)


### Bug Fixes

* **dispatcher:** bound model requests by the turn deadline instead of a fixed 45 seconds ([24b30c0](https://github.com/sympozium-ai/celln/commit/24b30c000a363dc9d227c9f074d0a76dad9eb024))
* **dispatcher:** bound model requests by the turn deadline instead of a fixed 45 seconds ([7be438c](https://github.com/sympozium-ai/celln/commit/7be438c08fee16805cfbb1d1a4f4c101db612591))

## [0.5.12](https://github.com/sympozium-ai/celln/compare/v0.5.11...v0.5.12) (2026-09-14)


### Bug Fixes

* **dispatcher:** charge exact broker slots per parent so a node holds more than one parent ([a1aa888](https://github.com/sympozium-ai/celln/commit/a1aa88808ad9970f7a48c2e5150ed59a0e763c00))
* **dispatcher:** charge exact broker slots per parent so a node holds more than one parent ([2341461](https://github.com/sympozium-ai/celln/commit/2341461a3d06b06d38aaca8a71f7fb36a2bca3f2))

## [0.5.11](https://github.com/sympozium-ai/celln/compare/v0.5.10...v0.5.11) (2026-09-14)


### Features

* **parents:** provision on the owning dispatcher; router binds affinity at provisioning ([#109](https://github.com/sympozium-ai/celln/issues/109)) ([bd17846](https://github.com/sympozium-ai/celln/commit/bd1784618ba011cddba4f559f80fb45437949539))
* **router:** re-resolve --backends-srv so owners can join or leave live ([#110](https://github.com/sympozium-ai/celln/issues/110)) ([4c08bcd](https://github.com/sympozium-ai/celln/commit/4c08bcdba1e50df8d0227a8925651367445592a0))

## [0.5.10](https://github.com/sympozium-ai/celln/compare/v0.5.9...v0.5.10) (2026-09-11)


### Features

* **router:** route /v1/parents with durable parent affinity; dispatcher drain ([#103](https://github.com/sympozium-ai/celln/issues/103)) ([f5403e5](https://github.com/sympozium-ai/celln/commit/f5403e507748985de740febd2e1b98c65461ae97))

## [0.5.9](https://github.com/sympozium-ai/celln/compare/v0.5.8...v0.5.9) (2026-09-11)


### Features

* **egress:** opt-in HTTP and self-signed model endpoints ([#101](https://github.com/sympozium-ai/celln/issues/101)) ([ccc6367](https://github.com/sympozium-ai/celln/commit/ccc63677643653e0687b139a5c48188df2802c1c))

## 0.5.8

### Added

- Native persistent Harness parents with disposable per-turn cells, bounded
  host-model access and explicitly borrowed workspace read/write and HTTPS tools.
- Operator-signed starter packaging, hardware admission and reviewed configuration
  commands. Linux amd64 archives now include the native parent, turn worker and
  starter tool binaries; a matching versioned router/provisioner image is published.

### Fixed

- Preserve execution ownership and receipts across authenticated router restarts.
- Confirm crashed native-owner cleanup from its recorded prelaunch process identity
  without replaying work or claiming restored context.

### Limits

- Persistent means live context, not crash recovery. Parent loss loses volatile
  context/files; old journals and host reboot cases remain conservatively fenced.
- Native starter support is Linux amd64/KVM only. Python, shell, arbitrary OCI
  Harness compatibility, checkpoints and pause/resume are not included.

## 0.5.7

### Added

- **Runs can declare an explicit environment.** `[run.env]` and `[agent.env]`
  pass a reviewed map to the workload after it enters its sealed image. The
  map is the complete workload environment: Celln never inherits ambient host
  variables into a cell.

### Fixed

- **OCI tools that require runtime environment variables can now run.** A
  trimmed Go distribution, for example, can declare
  `GOROOT = "/usr/local/go"` rather than failing because pilot previously
  launched every workload with an empty environment.

## 0.5.6

### Added

- **`celln agent` now runs a declared agent spec directly.**
  `celln agent cell.toml --prompt "…"` keeps the spec's policy and overrides
  its prompt, while `celln agent "…"` remains the inline form. This makes the
  agent entry point consistent whether policy lives in a file or in memory.

### Changed

- **Provider input is now called a prompt.** New specs use `[agent].prompt`
  and the CLI uses `--prompt`, which says what the value is instead of calling
  the same thing a task in the cell. Existing `[agent].task` and `--task`
  spellings remain accepted for compatibility.

### Fixed

- **A provider prompt could be silently ignored for a static spec.** Passing
  `--task` to a `[run]` spec used to execute its pinned empty or static argv;
  it now refuses and explains that a prompt requires `[agent]`.

## 0.5.5

### Added

- **A spec can now ask a provider to supply arguments for any declared tool.**
  `[agent]` no longer requires an interpreter: when its `exec` names one, the
  provider writes a program as before; otherwise it writes a JSON argv for the
  named tool. This makes a declared non-interpreter such as `curl` usable from
  a reviewed task spec. The CLI warns that model-authored argv retains the
  tool lane; use `[run]` to pin an invocation without a provider.

### Fixed

- **A cell could boot without `pilot` and then execute nothing.**
  `mkinitramfs.sh` previously printed that the guest supervisor was skipped
  yet returned success, leaving the useful cause buried behind a later
  `pilot=absent` guest report. Launch now checks that static guest assets can
  be packaged or the musl target can build them; the initramfs builder fails
  directly otherwise. Runtime setup also refuses to package a host-native
  `pilot`, since the stripped guest has no host dynamic loader.

- **Publishing a crate before a sibling it depends on silently shipped a
  version that cannot be installed.** Cargo resolves a requirement to the
  newest version satisfying it, so a dependent published ahead of its
  dependency does not fail — it succeeds, and breaks for whoever runs
  `cargo install`. Every inter-crate requirement was pinned at `0.5.0` while
  the workspace was four releases past it, which made this reachable at any
  time; 0.5.4 came within one command of it, with `celln-cli` calling a
  `celln-spec` function that published `celln-spec` did not have.

  Requirements now track the workspace version exactly, so a missing sibling
  is a publish-time refusal instead. They live in `[workspace.dependencies]`
  so there is one place to move them, `scripts/release.sh --bump` moves them
  with the version, and `--check` fails if one drifts — which ci runs on
  every PR. `--publish` derives its order from the dependency graph and waits
  for each crate to reach the index before the next.

### Changed

- **`celln agents` is now `celln providers`.** Those entries are inference
  backends — who *writes* a program — while "agent" already names what runs
  inside a cell and the lane it runs in. One word for both invited exactly the
  wrong reading of `celln agents`, which lists neither agents nor anything to
  do with the agent lane. `celln agent` is unchanged, and so is the `[agent]`
  spec block.

  Nothing breaks. `celln agents` still works as a hidden alias, `--agent`
  remains an alias for `--provider`, and `CELLN_AGENT` is still read
  (`CELLN_PROVIDER` takes precedence). The saved default moves from `[agent]`
  to `[provider]` in `config.toml`; an existing file is still read, and is
  rewritten to `[provider]` the next time the default is set.

  `--json` event names are deliberately unchanged — they are a machine
  contract, and renaming them belongs with a deliberate decision about
  consumers rather than riding along with a wording fix.

- The README's spec example moved below installation, so the page reads
  one-liner, what it is, install, then the durable form.

## 0.5.4

### Changed

- **Naming a tool the host does not have now says how to get one.** `celln
  agent --tool go` listed what was available and stopped there, which tells
  you the command failed but not what to do about it. It now gives the
  `celln image add` line, including the `--name` form for when a tool is
  published under a different name than you call it — `go` lives in `golang`.

- **Being told a tool cannot run model-written code now says why, and what
  to use instead.** The old message named a missing `language` and
  `code_flag` without explaining that `--tool` needs an interpreter taking a
  program on a flag, which plenty of useful tools have no reason to do. It
  now points at `celln image spec NAME`, which is how those are lent.

### Fixed

- **`celln image add` produced entries that `celln agent --tool` then
  refused.** It wrote `interpreter` but never `language` or `code_flag`, so
  adding an interpreter and immediately using it failed on a field the user
  was never told to write. A recognised interpreter now records the flag it
  takes code on, and adding one is enough to use it.

- **The weekly digest refresh could not have worked.** It exists so a moved
  upstream tag arrives as a reviewable PR rather than silently at pull time,
  and it had never run its own body — it is gated on a digest having moved,
  and none had. Three faults, which only made sense to fix together:

  - Its one verification ran `cargo test -p celln-cli --lib catalogue`, and
    `celln-cli` has no library target, so the command errors instead of
    running the tests. That step failing is the only thing that would have
    stopped the next two from reaching a pull request.
  - A registry answering with something that is not a digest — an empty
    string on a hiccup, `unauthorized` on a rate limit — was pinned verbatim,
    producing `ref = "docker.io/library/python@"`. No pull can satisfy that.
  - Stripping the tag off a reference ate the port of any registry that has
    one, turning `reg:5000/team/tool:v1` into `reg`.

  The refresher is now `scripts/refresh-tool-digests.sh` rather than a block
  of YAML, so it can be run and tested without a registry. Its `--self-test`
  stubs skopeo and covers all three, and runs in ci on every PR — the point
  being that weekly-only code is otherwise tested exclusively in production.

## 0.5.3

### Fixed

- **`celln image pull python` failed on a fresh store**, so a new install could
  not materialise the flagship tool. Every file occupies whole blocks, and
  `python:3.12-slim` is mostly small stdlib files, so summing file sizes built
  an image too small to hold its own contents: `Could not allocate block in
  ext2 filesystem`. Sizing now counts blocks and directories.

  A regression from 0.5.1. Deduping hardlinked inodes was correct — `mke2fs -d`
  preserves hardlinks — but it tightened the estimate enough to cross the line.
  Hosts that already had the image were unaffected.

- **A failed image build left a partial filesystem behind**, which `image list`
  reported as materialised and a spec could have sealed. It is removed on
  failure, and mke2fs's own error is surfaced rather than a generic one.

- **`celln setup` skipped tool images when no agent CLI was present**, returning
  before it reached them. Which model writes code has nothing to do with which
  tools a host can lend; the two are now independent, and the exit code still
  reports the missing backend.

- **A Kubernetes node never got its tool images.** The installer runs setup in
  the host namespace, where there is no skopeo. Agent config and runtime assets
  are now installed there with `--no-tools`, and images are materialised from
  inside the installer container — which carries skopeo — into the host store
  over the existing `/host` mount.

## 0.5.2

### Security

- **A declared interpreter could be ignored, running agent-authored code in the
  tool lane.** The laundering ban turns on `Entry::interpreter`, and
  `Assayer::resolve` used the caller's declaration only when admitting bytes it
  had not seen. On a warm hit it returned the stored entry and discarded the
  declaration — so if any spec had admitted a tool as a plain binary, every
  later spec that correctly marked it an interpreter was ignored, and
  agent-authored input ran with full tool-lane authority. Nothing warned.

  Interpreter-ness now only ever tightens: declaring it re-admits before
  anything runs, and declaring `false` cannot loosen an entry already marked.
  It is a property of the bytes, not of whoever admitted them first.

  Affects 0.5.0 and 0.5.1, and only a host whose store already held the tool
  as a non-interpreter — a fresh store admits the declared value correctly.

## 0.5.1

### Fixed

- `celln image list` showed a bare sha256 per image, which identifies nothing
  a person is trying to recall. It now shows the name the image was pulled
  under, its size, a shortened digest and the tag it was pinned from. An image
  whose catalogue entry has gone shows as `(untracked)` with its digest, so it
  can still be identified and cleaned up. JSON keeps the full digest and gains
  the name and tag.

## 0.5.0

Celln could seal and attest any file, but only ever *run* one shape of thing: a
static musl binary it built itself. Real tools are not that shape — a `python`
is a binary plus a loader plus a tree of shared objects resolved by absolute
path, and on a working developer machine 3 of 2064 binaries in `/usr/bin` are
static. This release lends a tool's whole dependency closure instead, as a
sealed filesystem built from a digest-pinned OCI image.

### Breaking

- **`celln ask` is removed.** It sent a question to the configured model CLI on
  the host — no cell, no tools, no attestation. Use that CLI directly.
- **`Tool.path` is now optional.** A tool comes from exactly one of `path`
  (a static binary on this host), `image` + `exec` (a dependency closure), or
  `builtin = "fetch"`. Specs setting none, or more than one, are refused.

### Added

- **Images as tools.** `[[tool]] image = "python"` with `exec`, pinned by
  digest; tags are refused, because a moved tag would change what a cell is
  lent without the spec changing.
- **A tool catalogue**, compiled into the binary and refreshed by CI.
  `celln image add <image:tag>` resolves the digest, materialises the image,
  inspects it without mounting, and exposes what it finds. Hosts extend it at
  `<root>/tools.toml` without rebuilding; a local entry shadows a shipped one.
  Also `celln image pull|list|catalogue|spec|remove`.
- **`celln run` executes.** It previously sealed tools and dissolved without
  running anything.
- **Several images per cell.** Each becomes its own pmem namespace mounted at
  `/tools`, `/tools1`, …; tools naming the same image share its mount and its
  single physical copy.
- **Several invocations per cell** via `[[run]]`. `[run]` still takes one.
- **Brokered egress from a spec**: `[cell] allow_hosts` with a tool declaring
  `builtin = "fetch"`. The cell still has no network stack — the host performs
  the fetch, HTTPS only, DNS pinned before connect, each redirect
  re-authorised, size and time bounded.
- **`[agent]` blocks.** The spec keeps the policy — tools, memory, hosts — and
  a model fills in the program. `celln run --task` overrides the task;
  `celln agent --tool python "…"` does the same without a file.
- **`celln tools`** lists what the host has attested rather than a count.

### Fixed

- **The VFS↔memslot proofs were red.** The Celln rename widened `PROBE_MAGIC`
  from 8 bytes to 9 *and* relaxed its type from `&[u8; 8]` to `&[u8]`, turning
  two compile-checked lengths into runtime bugs. The join this design rests on
  was unverified.
- **Sealed images mounted read-write**, because the read-only remount sat
  behind an early return taken whenever a test fixture was absent. Writes
  returned success, appeared to create files, and landed nowhere.
- **A warm hit matched on alias, not content.** Two images can both claim
  `/bin/sh`; the host attested one image's bytes while the other's ran.
- **`Manifest::resolve_alias` returned revoked entries.**
- **Image sizing counted hardlinks repeatedly** — busybox links ~400 applets to
  one binary, so a 4 MiB rootfs was sized as ~400 MiB.
- **Scratch directories leaked per run** into `/tmp`, image-sized, and `/tmp` is
  a tmpfs on most hosts.
- **The tool window capped images at 32 MiB.** It is now sized to the image.
  Moving it above RAM does not work: pmem past `last_pfn` is parsed and then
  never registered, so no device appears.

### Known limits

Images are capped at 512 MiB; past roughly a gigabyte the guest panics in
`kernel_init`. `curl` deliberately cannot use the fetch capability — reaching it
through curl means brokering raw TCP rather than a validated URL, which discards
the DNS pinning and redirect re-authorisation that make it safe. File ownership
in built images is still the extracting user's, and cell scratch still lives in
`/tmp`. See `docs/OCI_TOOL_LANE.md`.
