// Generated macro for select_priv_clean_pattern (function)
macro_rules! Depcrateselect_priv_clean_pattern {
() => {
// Module: crate
// Provides: {"select_priv_clean_pattern"}
// Dependencies: {}
# [doc = " Implementation detail of the `select!` macro. This macro is **not** intended"] # [doc = " to be used as part of the public API and is permitted to change."] # [proc_macro] # [doc (hidden)] pub fn select_priv_clean_pattern (input : TokenStream) -> TokenStream { select :: clean_pattern_macro (input) }
};
}
