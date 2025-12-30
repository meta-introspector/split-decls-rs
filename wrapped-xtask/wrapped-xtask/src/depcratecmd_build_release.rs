// Generated macro for cmd_build_release (function)
macro_rules! Depcratecmd_build_release {
() => {
// Module: crate
// Provides: {"cmd_build_release"}
// Dependencies: {}
fn cmd_build_release () -> Result < () , DynError > { cargo (& ["build" , "--workspace" , "--features" , "all" , "--release"]) }
};
}
