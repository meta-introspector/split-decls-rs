// Generated macro for to_close (function)
macro_rules! Depcrate_to_tokens_attributeto_close {
() => {
// Module: crate::to_tokens_attribute
// Provides: {"to_close"}
// Dependencies: {}
pub fn to_close (c : char) -> char { match c { '(' => ')' , '[' => ']' , '{' => '}' , _ => panic ! ("not found closing delimiter for {c}") , } }
};
}
