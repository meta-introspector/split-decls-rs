// Generated macro for parse_optional_keyword (function)
macro_rules! Depcrate_tokenparse_optional_keyword {
() => {
// Module: crate::token
// Provides: {"parse_optional_keyword"}
// Dependencies: {}
pub fn parse_optional_keyword (iter : Iter , keyword : & str) -> Option < Span > { match iter . peek () { Some (TokenTree :: Ident (ident)) if ident . to_string () == keyword => { Some (iter . next () . unwrap () . span ()) } _ => None , } }
};
}
