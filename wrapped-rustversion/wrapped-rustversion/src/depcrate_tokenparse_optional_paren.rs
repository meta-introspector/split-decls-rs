// Generated macro for parse_optional_paren (function)
macro_rules! Depcrate_tokenparse_optional_paren {
() => {
// Module: crate::token
// Provides: {"parse_optional_paren"}
// Dependencies: {}
pub fn parse_optional_paren (iter : Iter) -> Option < Group > { match iter . peek () { Some (TokenTree :: Group (group)) if group . delimiter () == Delimiter :: Parenthesis => { match iter . next () { Some (TokenTree :: Group (group)) => Some (group) , _ => unreachable ! () , } } _ => None , } }
};
}
