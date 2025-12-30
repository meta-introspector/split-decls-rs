// Generated macro for debug (macro)
macro_rules! Depcrate_tracingdebug {
() => {
// Module: crate::tracing
// Provides: {"debug"}
// Dependencies: {}
macro_rules ! debug { ($ ($ x : tt) *) => { crate :: tracing :: event ! (DEBUG , $ ($ x) *) } ; }
};
}
