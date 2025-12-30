// Generated macro for exit_status (function)
macro_rules! Depcrate_distexit_status {
() => {
// Module: crate::dist
// Provides: {"exit_status"}
// Dependencies: {}
# [cfg (windows)] fn exit_status (code : i32) -> process :: ExitStatus { process :: ExitStatus :: from_raw (code as u32) }
};
}
