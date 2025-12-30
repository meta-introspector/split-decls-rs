// Generated macro for quote_group (macro)
macro_rules! Depcrate_quotequote_group {
() => {
// Module: crate::quote
// Provides: {"quote_group"}
// Dependencies: {}
macro_rules ! quote_group { ({ $ ($ x : tt) * }) => { { use proc_macro ::*; TokenTree :: Group (Group :: new (Delimiter :: Brace , quote_ ! ($ ($ x) *))) } } ; }
};
}
