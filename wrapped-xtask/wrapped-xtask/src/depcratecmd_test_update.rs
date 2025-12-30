// Generated macro for cmd_test_update (function)
macro_rules! Depcratecmd_test_update {
() => {
// Module: crate
// Provides: {"cmd_test_update"}
// Dependencies: {}
fn cmd_test_update () -> Result < () , DynError > { cargo_with (& ["test" , "--workspace" , "--features" , "all"] , | cmd | { cmd . env ("OBJECT_TESTFILES_UPDATE" , "1") ; }) }
};
}
