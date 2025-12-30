// Generated macro for span_err (function)
macro_rules! Depcrate_diagnostics_errorspan_err {
() => {
// Module: crate::diagnostics::error
// Provides: {"span_err"}
// Dependencies: {}
# [doc = " Returns an error diagnostic on span `span` with msg `msg`."] # [must_use] pub (crate) fn span_err < T : Into < String > > (span : impl MultiSpan , msg : T) -> Diagnostic { Diagnostic :: spanned (span , Level :: Error , format ! ("derive(Diagnostic): {}" , msg . into ())) }
};
}
