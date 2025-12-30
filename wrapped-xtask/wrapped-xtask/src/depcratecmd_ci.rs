// Generated macro for cmd_ci (function)
macro_rules! Depcratecmd_ci {
() => {
// Module: crate
// Provides: {"cmd_ci"}
// Dependencies: {}
fn cmd_ci () -> Result < () , DynError > { cmd_check () ? ; cmd_test () ? ; cmd_features () ? ; cmd_cross () ? ; cmd_msrv () ? ; cmd_fmt () ? ; cmd_doc () ? ; Ok (()) }
};
}
