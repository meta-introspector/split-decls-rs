// Generated macro for trace (macro)
macro_rules! Depcrate_tracingtrace {
() => {
// Module: crate::tracing
// Provides: {"trace"}
// Dependencies: {}
macro_rules ! trace { ($ ($ x : tt) *) => { crate :: tracing :: event ! (TRACE , $ ($ x) *) } ; }
};
}
