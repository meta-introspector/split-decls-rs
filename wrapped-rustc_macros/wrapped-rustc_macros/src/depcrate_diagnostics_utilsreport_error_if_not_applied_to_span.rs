// Generated macro for report_error_if_not_applied_to_span (function)
macro_rules! Depcrate_diagnostics_utilsreport_error_if_not_applied_to_span {
() => {
// Module: crate::diagnostics::utils
// Provides: {"report_error_if_not_applied_to_span"}
// Dependencies: {}
# [doc = " Reports an error if the field's type is not `Span`."] pub (crate) fn report_error_if_not_applied_to_span (attr : & Attribute , info : & FieldInfo < '_ > ,) -> Result < () , DiagnosticDeriveError > { if ! type_matches_path (info . ty . inner_type () , & ["rustc_span" , "Span"]) && ! type_matches_path (info . ty . inner_type () , & ["rustc_errors" , "MultiSpan"]) { report_type_error (attr , "`Span` or `MultiSpan`") ? ; } Ok (()) }
};
}
