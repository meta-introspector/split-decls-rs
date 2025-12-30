// Generated macro for parse_visibility (function)
macro_rules! Depcrateparse_visibility {
() => {
// Module: crate
// Provides: {"parse_visibility"}
// Dependencies: {}
# [cfg (all (feature = "serde" , any (feature = "formatting" , feature = "parsing")))] fn parse_visibility (iter : & mut PeekableTokenStreamIter) -> Result < TokenStream , Error > { let mut visibility = match iter . peek () . ok_or (Error :: UnexpectedEndOfInput) ? { pub_ident @ TokenTree :: Ident (ident) if ident . to_string () == "pub" => { let visibility = quote_ ! { # (pub_ident . clone ()) } ; iter . next () ; visibility } _ => return Ok (quote_ ! { }) , } ; match iter . peek () . ok_or (Error :: UnexpectedEndOfInput) ? { group @ TokenTree :: Group (path) if path . delimiter () == Delimiter :: Parenthesis => { visibility . extend (std :: iter :: once (group . clone ())) ; iter . next () ; } _ => { } } Ok (visibility) }
};
}
