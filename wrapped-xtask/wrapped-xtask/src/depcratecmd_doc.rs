// Generated macro for cmd_doc (function)
macro_rules! Depcratecmd_doc {
() => {
// Module: crate
// Provides: {"cmd_doc"}
// Dependencies: {}
fn cmd_doc () -> Result < () , DynError > { cargo_with (& ["doc" , "--workspace" , "--lib" , "--no-default-features" , "--features" , "doc" ,] , | cmd | { cmd . env ("RUSTDOCFLAGS" , "-D warnings") ; } ,) }
};
}
