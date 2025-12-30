// Generated macro for bail (macro)
macro_rules! Depcrate_syn_utilsbail {
() => {
// Module: crate::syn_utils
// Provides: {"bail"}
// Dependencies: {}
macro_rules ! bail { (_ , $ ($ arg : tt) *) => { bail ! (:: proc_macro2 :: Span :: call_site () , $ ($ arg) *) } ; ($ span : expr , $ fmt : literal $ (,) ?) => { return :: std :: result :: Result :: Err (:: syn :: Error :: new ($ span , :: std :: format ! ($ fmt))) } ; ($ span : expr , $ fmt : literal , $ ($ arg : tt) *) => { return :: std :: result :: Result :: Err (:: syn :: Error :: new ($ span , :: std :: format ! ($ fmt , $ ($ arg) *))) } ; }
};
}
