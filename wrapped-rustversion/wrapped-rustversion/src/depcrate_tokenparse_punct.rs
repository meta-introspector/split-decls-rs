// Generated macro for parse_punct (function)
macro_rules! Depcrate_tokenparse_punct {
() => {
// Module: crate::token
// Provides: {"parse_punct"}
// Dependencies: {}
pub fn parse_punct (iter : Iter , ch : char) -> Result < () > { match iter . next () { Some (TokenTree :: Punct (ref punct)) if punct . as_char () == ch => Ok (()) , unexpected => { let span = unexpected . as_ref () . map_or_else (Span :: call_site , TokenTree :: span) ; Err (Error :: new (span , format ! ("expected `{}`" , ch))) } } }
};
}
