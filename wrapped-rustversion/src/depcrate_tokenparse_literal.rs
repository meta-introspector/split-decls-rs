// Generated macro for parse_literal (function)
macro_rules! Depcrate_tokenparse_literal {
() => {
// Module: crate::token
// Provides: {"parse_literal"}
// Dependencies: {}
pub fn parse_literal (iter : Iter) -> Result < Literal > { match iter . next () { Some (TokenTree :: Literal (literal)) => Ok (literal) , unexpected => { let span = unexpected . as_ref () . map_or_else (Span :: call_site , TokenTree :: span) ; Err (Error :: new (span , "expected literal")) } } }
};
}
