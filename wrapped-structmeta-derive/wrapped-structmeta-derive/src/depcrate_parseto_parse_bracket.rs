// Generated macro for to_parse_bracket (function)
macro_rules! Depcrate_parseto_parse_bracket {
() => {
// Module: crate::parse
// Provides: {"to_parse_bracket"}
// Dependencies: {}
fn to_parse_bracket (c : char) -> Ident { match c { '(' => parse_quote ! (parenthesized) , '[' => parse_quote ! (bracketed) , '{' => parse_quote ! (braced) , _ => unreachable ! () , } }
};
}
