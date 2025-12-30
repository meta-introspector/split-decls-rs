// Generated macro for flag_infer_long_and_short (function)
macro_rules! Depcrate_cmdlineflag_infer_long_and_short {
() => {
// Module: crate::cmdline
// Provides: {"flag_infer_long_and_short"}
// Dependencies: {}
fn flag_infer_long_and_short (name : & 'static str) -> Arg { flag_infer_long (name) . short (name . chars () . next () . expect ("Name needs at least one char")) }
};
}
