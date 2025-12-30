// Generated macro for warn_event (macro)
macro_rules! Depcrate_tracingwarn_event {
() => {
// Module: crate::tracing
// Provides: {"warn_event"}
// Dependencies: {}
macro_rules ! warn_event { ($ ($ x : tt) *) => { crate :: tracing :: event ! (WARN , $ ($ x) *) } ; }
};
}
