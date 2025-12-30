// Generated macro for quote_ (macro)
macro_rules! Depcrate_quotequote_ {
() => {
// Module: crate::quote
// Provides: {"quote_"}
// Dependencies: {}
macro_rules ! quote_ { () => (proc_macro :: TokenStream :: new ()) ; ($ ($ x : tt) *) => { { use proc_macro ::*; let mut ts = TokenStream :: new () ; let ts_mut = & mut ts ; quote_inner ! (ts_mut $ ($ x) *) ; ts } } ; }
};
}
