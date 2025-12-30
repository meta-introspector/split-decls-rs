// Generated macro for cmd_build (function)
macro_rules! Depcratecmd_build {
() => {
// Module: crate
// Provides: {"cmd_build"}
// Dependencies: {}
fn cmd_build () -> Result < () , DynError > { cargo (& ["build" , "--workspace" , "--features" , "all"]) }
};
}
