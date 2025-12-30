// Generated macro for _throw_err (function)
macro_rules! Depcrate_diagnostics_error_throw_err {
() => {
// Module: crate::diagnostics::error
// Provides: {"_throw_err"}
// Dependencies: {}
# [doc = " Helper function for use with `throw_*` macros - constraints `$f` to an `impl FnOnce`."] pub (crate) fn _throw_err (diag : Diagnostic , f : impl FnOnce (Diagnostic) -> Diagnostic ,) -> DiagnosticDeriveError { f (diag) . emit () ; DiagnosticDeriveError :: ErrorHandled }
};
}
