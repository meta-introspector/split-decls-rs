// Generated macro for cmd_test (function)
macro_rules! Depcratecmd_test {
() => {
// Module: crate
// Provides: {"cmd_test"}
// Dependencies: {}
fn cmd_test () -> Result < () , DynError > { cargo (& ["test" , "--workspace" , "--features" , "all"]) }
};
}
