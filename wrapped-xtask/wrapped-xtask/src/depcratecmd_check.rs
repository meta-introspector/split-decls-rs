// Generated macro for cmd_check (function)
macro_rules! Depcratecmd_check {
() => {
// Module: crate
// Provides: {"cmd_check"}
// Dependencies: {}
fn cmd_check () -> Result < () , DynError > { cargo (& ["check" , "--workspace" , "--features" , "all"]) }
};
}
