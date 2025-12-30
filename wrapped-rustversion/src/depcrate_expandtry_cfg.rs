// Generated macro for try_cfg (function)
macro_rules! Depcrate_expandtry_cfg {
() => {
// Module: crate::expand
// Provides: {"try_cfg"}
// Dependencies: {}
fn try_cfg (introducer : & str , args : TokenStream , input : TokenStream) -> Result < TokenStream > { let introducer = Ident :: new (introducer , Span :: call_site ()) ; let mut full_args = TokenStream :: from (TokenTree :: Ident (introducer)) ; if ! args . is_empty () { full_args . extend (std :: iter :: once (TokenTree :: Group (Group :: new (Delimiter :: Parenthesis , args ,)))) ; } let ref mut full_args = iter :: new (full_args) ; let expr = expr :: parse (full_args) ? ; token :: parse_end (full_args) ? ; if expr . eval (crate :: RUSTVERSION) { Ok (allow_incompatible_msrv (input)) } else { Ok (TokenStream :: new ()) } }
};
}
