// Generated macro for report_error_if_not_applied_to_applicability (function)
macro_rules! Depcrate_diagnostics_utilsreport_error_if_not_applied_to_applicability {
() => {
// Module: crate::diagnostics::utils
// Provides: {"report_error_if_not_applied_to_applicability"}
// Dependencies: {}
# [doc = " Reports an error if the field's type is not `Applicability`."] pub (crate) fn report_error_if_not_applied_to_applicability (attr : & Attribute , info : & FieldInfo < '_ > ,) -> Result < () , DiagnosticDeriveError > { report_error_if_not_applied_to_ty (attr , info , & ["rustc_errors" , "Applicability"] , "`Applicability`" ,) }
};
}
