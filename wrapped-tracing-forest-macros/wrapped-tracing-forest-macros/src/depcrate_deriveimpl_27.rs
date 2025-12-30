// Generated macro for impl_27 (impl)
macro_rules! Depcrate_deriveimpl_27 {
() => {
// Module: crate::derive
// Provides: {"impl_27"}
// Dependencies: {}
impl Level { fn quote (& self) -> TokenStream2 { match self { Level :: Trace => quote ! { trace } , Level :: Debug => quote ! { debug } , Level :: Info => quote ! { info } , Level :: Warn => quote ! { warn } , Level :: Error => quote ! { error } , } } fn quote_icon (& self) -> TokenStream2 { let constant = match self { Level :: Trace => quote ! { TRACE_ICON } , Level :: Debug => quote ! { DEBUG_ICON } , Level :: Info => quote ! { INFO_ICON } , Level :: Warn => quote ! { WARN_ICON } , Level :: Error => quote ! { ERROR_ICON } , } ; quote ! { :: tracing_forest :: private ::# constant } } }
};
}
