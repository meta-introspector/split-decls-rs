// Generated macro for DocTestUnknownInclude (struct)
macro_rules! Depcrate_errorsDocTestUnknownInclude {
() => {
// Module: crate::errors
// Provides: {"DocTestUnknownInclude"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_include)] pub (crate) struct DocTestUnknownInclude { pub path : String , pub value : String , pub inner : & 'static str , # [suggestion (code = "#{inner}[doc = include_str!(\"{value}\")]")] pub sugg : (Span , Applicability) , }
};
}
