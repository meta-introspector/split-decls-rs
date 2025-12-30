// Generated macro for parse_paren (function)
macro_rules! Depcrate_tokenparse_paren {
() => {
// Module: crate::token
// Provides: {"parse_paren"}
// Dependencies: {}
pub fn parse_paren (introducer : & Ident , iter : Iter) -> Result < Group > { match iter . peek () { Some (TokenTree :: Group (group)) if group . delimiter () == Delimiter :: Parenthesis => { match iter . next () { Some (TokenTree :: Group (group)) => Ok (group) , _ => unreachable ! () , } } Some (unexpected) => Err (Error :: new (unexpected . span () , "expected `(`")) , None => Err (Error :: new (introducer . span () , format ! ("expected `(` after `{}`" , introducer) ,)) , } }
};
}
