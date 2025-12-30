// Generated macro for collect_lifetimes_from_tokens (function)
macro_rules! Depcrate_internals_attrcollect_lifetimes_from_tokens {
() => {
// Module: crate::internals::attr
// Provides: {"collect_lifetimes_from_tokens"}
// Dependencies: {}
fn collect_lifetimes_from_tokens (tokens : TokenStream , out : & mut BTreeSet < syn :: Lifetime >) { let mut iter = tokens . into_iter () ; while let Some (tt) = iter . next () { match & tt { TokenTree :: Punct (op) if op . as_char () == '\'' && op . spacing () == Spacing :: Joint => { if let Some (TokenTree :: Ident (ident)) = iter . next () { out . insert (syn :: Lifetime { apostrophe : op . span () , ident , }) ; } } TokenTree :: Group (group) => { let tokens = group . stream () ; collect_lifetimes_from_tokens (tokens , out) ; } _ => { } } } }
};
}
