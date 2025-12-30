// Generated macro for look_up_flag_from_table (function)
macro_rules! Depcrate_cmdlinelook_up_flag_from_table {
() => {
// Module: crate::cmdline
// Provides: {"look_up_flag_from_table"}
// Dependencies: {}
fn look_up_flag_from_table (flag : & str) -> Option < FlagType > { KNOWN_FLAGS . iter () . cloned () . filter (| & (name , _) | name == flag) . map (| (_ , typ) | typ) . next () }
};
}
