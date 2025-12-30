// Generated macro for trace (macro)
macro_rules! Depcratetrace {
() => {
// Module: crate
// Provides: {"trace"}
// Dependencies: {}
# [macro_export] macro_rules ! trace { ($ ($ arg : tt) *) => { # [cfg (feature = "ZLIB_DEBUG")] { eprint ! ($ ($ arg) *) } } ; }
};
}
