// Generated macro for is_escapable (function)
macro_rules! Depcrate_lexeris_escapable {
() => {
// Module: crate::lexer
// Provides: {"is_escapable"}
// Dependencies: {}
fn is_escapable (c : char) -> bool { matches ! (c , '\\' | '\'') }
};
}
