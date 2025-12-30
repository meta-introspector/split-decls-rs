// Generated macro for push_idents2 (function)
macro_rules! Depcrate_hygienepush_idents2 {
() => {
// Module: crate::hygiene
// Provides: {"push_idents2"}
// Dependencies: {}
fn push_idents2 (input : proc_macro2 :: TokenStream , user_tokens : & mut HashSet < String >) { input . into_iter () . for_each (| token | match token { proc_macro2 :: TokenTree :: Group (g) => { push_idents2 (g . stream () , user_tokens) ; } proc_macro2 :: TokenTree :: Ident (ident) => { user_tokens . insert (ident . to_string ()) ; } proc_macro2 :: TokenTree :: Punct (_) => () , proc_macro2 :: TokenTree :: Literal (_) => () , }) }
};
}
