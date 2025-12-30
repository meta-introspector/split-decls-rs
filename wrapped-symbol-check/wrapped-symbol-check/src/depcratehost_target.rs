// Generated macro for host_target (function)
macro_rules! Depcratehost_target {
() => {
// Module: crate
// Provides: {"host_target"}
// Dependencies: {}
fn host_target () -> String { let out = Command :: new ("rustc") . arg ("--version") . arg ("--verbose") . output () . unwrap () ; assert ! (out . status . success ()) ; let out = String :: from_utf8 (out . stdout) . unwrap () ; out . lines () . find_map (| s | s . strip_prefix ("host: ")) . unwrap () . to_owned () }
};
}
