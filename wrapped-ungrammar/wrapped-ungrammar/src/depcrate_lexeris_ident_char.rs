// Generated macro for is_ident_char (function)
macro_rules! Depcrate_lexeris_ident_char {
() => {
// Module: crate::lexer
// Provides: {"is_ident_char"}
// Dependencies: {}
fn is_ident_char (c : char) -> bool { matches ! (c , 'a' ..='z' | 'A' ..='Z' | '_') }
};
}
