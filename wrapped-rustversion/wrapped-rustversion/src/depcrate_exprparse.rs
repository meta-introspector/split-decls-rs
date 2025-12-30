// Generated macro for parse (function)
macro_rules! Depcrate_exprparse {
() => {
// Module: crate::expr
// Provides: {"parse"}
// Dependencies: {}
pub fn parse (iter : Iter) -> Result < Expr > { match & iter . next () { Some (TokenTree :: Ident (i)) if i . to_string () == "stable" => parse_stable (iter) , Some (TokenTree :: Ident (i)) if i . to_string () == "beta" => Ok (Expr :: Beta) , Some (TokenTree :: Ident (i)) if i . to_string () == "nightly" => parse_nightly (iter) , Some (TokenTree :: Ident (i)) if i . to_string () == "since" => parse_since (i , iter) , Some (TokenTree :: Ident (i)) if i . to_string () == "before" => parse_before (i , iter) , Some (TokenTree :: Ident (i)) if i . to_string () == "not" => parse_not (i , iter) , Some (TokenTree :: Ident (i)) if i . to_string () == "any" => parse_any (i , iter) , Some (TokenTree :: Ident (i)) if i . to_string () == "all" => parse_all (i , iter) , unexpected => { let span = unexpected . as_ref () . map_or_else (Span :: call_site , TokenTree :: span) ; Err (Error :: new (span , "expected one of `stable`, `beta`, `nightly`, `since`, `before`, `not`, `any`, `all`")) } } }
};
}
