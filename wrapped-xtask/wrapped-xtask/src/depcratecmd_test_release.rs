// Generated macro for cmd_test_release (function)
macro_rules! Depcratecmd_test_release {
() => {
// Module: crate
// Provides: {"cmd_test_release"}
// Dependencies: {}
fn cmd_test_release () -> Result < () , DynError > { cargo (& ["test" , "--workspace" , "--features" , "all" , "--release"]) }
};
}
