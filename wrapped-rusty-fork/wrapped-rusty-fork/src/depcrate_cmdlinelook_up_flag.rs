// Generated macro for look_up_flag (function)
macro_rules! Depcrate_cmdlinelook_up_flag {
() => {
// Module: crate::cmdline
// Provides: {"look_up_flag"}
// Dependencies: {}
fn look_up_flag (flag : & str) -> Option < FlagType > { look_up_flag_from_table (flag) . or_else (| | look_up_flag_from_env (flag)) }
};
}
