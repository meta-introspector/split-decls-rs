// Generated macro for debug_span (macro)
macro_rules! Depcrate_tracingdebug_span {
() => {
// Module: crate::tracing
// Provides: {"debug_span"}
// Dependencies: {}
macro_rules ! debug_span { ($ ($ x : tt) *) => { crate :: tracing :: span ! (DEBUG , $ ($ x) *) } ; }
};
}
