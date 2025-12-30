// Generated macro for MixedExportNameAndNoMangle (struct)
macro_rules! Depcrate_errorsMixedExportNameAndNoMangle {
() => {
// Module: crate::errors
// Provides: {"MixedExportNameAndNoMangle"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (passes_mixed_export_name_and_no_mangle)] pub (crate) struct MixedExportNameAndNoMangle { # [label] # [suggestion (style = "verbose" , code = "" , applicability = "machine-applicable")] pub no_mangle_span : Span , # [note] pub export_name_span : Span , pub no_mangle_attr : & 'static str , pub export_name_attr : & 'static str , }
};
}
