// Generated macro for bail (macro)
macro_rules! Depcrate_syn_utilsbail {
() => {
// Module: crate::syn_utils
// Provides: {"bail"}
// Dependencies: {}
macro_rules ! bail { ($ span : expr , $ message : literal $ (,) ?) => { return std :: result :: Result :: Err (syn :: Error :: new ($ span , $ message)) } ; ($ span : expr , $ err : expr $ (,) ?) => { return std :: result :: Result :: Err (syn :: Error :: new ($ span , $ err)) } ; ($ span : expr , $ fmt : expr , $ ($ arg : tt) *) => { return std :: result :: Result :: Err (syn :: Error :: new ($ span , std :: format ! ($ fmt , $ ($ arg) *))) } ; }
};
}
