// Generated macro for close_char_of (function)
macro_rules! Depcrate_to_tokensclose_char_of {
() => {
// Module: crate::to_tokens
// Provides: {"close_char_of"}
// Dependencies: {}
fn close_char_of (delimiter : Delimiter) -> char { match delimiter { Delimiter :: Bracket => ']' , Delimiter :: Brace => '}' , Delimiter :: Parenthesis => ')' , _ => unreachable ! ("unsupported delimiter") , } }
};
}
