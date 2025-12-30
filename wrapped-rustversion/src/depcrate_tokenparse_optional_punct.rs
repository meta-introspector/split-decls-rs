// Generated macro for parse_optional_punct (function)
macro_rules! Depcrate_tokenparse_optional_punct {
() => {
// Module: crate::token
// Provides: {"parse_optional_punct"}
// Dependencies: {}
pub fn parse_optional_punct (iter : Iter , ch : char) -> Option < () > { match iter . peek () { Some (TokenTree :: Punct (punct)) if punct . as_char () == ch => iter . next () . map (drop) , _ => None , } }
};
}
