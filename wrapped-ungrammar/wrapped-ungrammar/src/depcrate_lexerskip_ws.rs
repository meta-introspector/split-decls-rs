// Generated macro for skip_ws (function)
macro_rules! Depcrate_lexerskip_ws {
() => {
// Module: crate::lexer
// Provides: {"skip_ws"}
// Dependencies: {}
fn skip_ws (input : & mut & str) { * input = input . trim_start_matches (is_whitespace) }
};
}
