// Generated macro for status_signal (function)
macro_rules! Depcrate_commandsstatus_signal {
() => {
// Module: crate::commands
// Provides: {"status_signal"}
// Dependencies: {}
# [doc = " Not implemented for non-Unix."] # [cfg (not (unix))] # [allow (dead_code)] fn status_signal (_status : process :: ExitStatus) -> Option < i32 > { None }
};
}
