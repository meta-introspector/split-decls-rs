// Generated macro for impl_13 (impl)
macro_rules! Depcrate_debugimpl_13 {
() => {
// Module: crate::debug
// Provides: {"impl_13"}
// Dependencies: {}
impl ErrorSink for DebugErrorSink < '_ > { fn report_error (& mut self , error : crate :: ParseError) { render_event (error . unexpected () , & format ! ("{error:?}") , anstyle :: AnsiColor :: Red . on_default () ,) ; self . sink . report_error (error) ; } }
};
}
