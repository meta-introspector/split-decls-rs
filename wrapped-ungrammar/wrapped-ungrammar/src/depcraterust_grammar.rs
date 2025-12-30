// Generated macro for rust_grammar (function)
macro_rules! Depcraterust_grammar {
() => {
// Module: crate
// Provides: {"rust_grammar"}
// Dependencies: {}
# [doc = " Returns a Rust grammar."] pub fn rust_grammar () -> Grammar { let src = include_str ! ("../rust.ungram") ; src . parse () . unwrap () }
};
}
