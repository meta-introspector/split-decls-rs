// Generated macro for find_ident (function)
macro_rules! Depcratefind_ident {
() => {
// Module: crate
// Provides: {"find_ident"}
// Dependencies: {}
fn find_ident (iter : & mut impl Iterator < Item = TokenTree >) -> Option < Ident > { match iter . next () ? { TokenTree :: Ident (i) => Some (i) , TokenTree :: Group (g) if g . delimiter () == Delimiter :: None => { find_ident (& mut g . stream () . into_iter ()) } _ => None , } }
};
}
