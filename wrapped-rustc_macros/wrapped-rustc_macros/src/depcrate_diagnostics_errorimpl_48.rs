// Generated macro for impl_48 (impl)
macro_rules! Depcrate_diagnostics_errorimpl_48 {
() => {
// Module: crate::diagnostics::error
// Provides: {"impl_48"}
// Dependencies: {}
impl DiagnosticDeriveError { pub (crate) fn to_compile_error (self) -> TokenStream { match self { DiagnosticDeriveError :: SynError (e) => e . to_compile_error () , DiagnosticDeriveError :: ErrorHandled => { quote ! { { unreachable ! () ; } } } } } }
};
}
