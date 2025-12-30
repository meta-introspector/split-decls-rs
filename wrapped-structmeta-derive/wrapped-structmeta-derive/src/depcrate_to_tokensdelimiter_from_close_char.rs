// Generated macro for delimiter_from_close_char (function)
macro_rules! Depcrate_to_tokensdelimiter_from_close_char {
() => {
// Module: crate::to_tokens
// Provides: {"delimiter_from_close_char"}
// Dependencies: {}
fn delimiter_from_close_char (value : char) -> Option < Delimiter > { match value { ']' => Some (Delimiter :: Bracket) , '}' => Some (Delimiter :: Brace) , ')' => Some (Delimiter :: Parenthesis) , _ => None , } }
};
}
