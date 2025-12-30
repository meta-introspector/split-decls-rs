// Generated macro for span (macro)
macro_rules! Depcrate_tracingspan {
() => {
// Module: crate::tracing
// Provides: {"span"}
// Dependencies: {}
macro_rules ! span { ($ level : ident , $ ($ x : tt) *) => { { let span = { # [cold] # [inline (never)] || { :: tracing :: span ! (:: tracing :: Level ::$ level , $ ($ x) *) } } ; if :: tracing :: enabled ! (:: tracing :: Level ::$ level) { span () } else { :: tracing :: Span :: none () } } } ; }
};
}
