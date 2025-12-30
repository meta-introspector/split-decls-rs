// Generated macro for info (macro)
macro_rules! Depcrate_tracinginfo {
() => {
// Module: crate::tracing
// Provides: {"info"}
// Dependencies: {}
macro_rules ! info { ($ ($ x : tt) *) => { crate :: tracing :: event ! (INFO , $ ($ x) *) } ; }
};
}
