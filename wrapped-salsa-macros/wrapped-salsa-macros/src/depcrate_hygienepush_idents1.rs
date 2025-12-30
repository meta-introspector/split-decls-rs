// Generated macro for push_idents1 (function)
macro_rules! Depcrate_hygienepush_idents1 {
() => {
// Module: crate::hygiene
// Provides: {"push_idents1"}
// Dependencies: {}
fn push_idents1 (input : proc_macro :: TokenStream , user_tokens : & mut HashSet < String >) { input . into_iter () . for_each (| token | match token { proc_macro :: TokenTree :: Group (g) => { push_idents1 (g . stream () , user_tokens) ; } proc_macro :: TokenTree :: Ident (ident) => { user_tokens . insert (ident . to_string ()) ; } proc_macro :: TokenTree :: Punct (_) => () , proc_macro :: TokenTree :: Literal (_) => () , }) }
};
}
