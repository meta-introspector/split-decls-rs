// Generated macro for MultipleRustcMain (struct)
macro_rules! Depcrate_errorsMultipleRustcMain {
() => {
// Module: crate::errors
// Provides: {"MultipleRustcMain"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_multiple_rustc_main , code = E0137)] pub (crate) struct MultipleRustcMain { # [primary_span] pub span : Span , # [label (passes_first)] pub first : Span , # [label (passes_additional)] pub additional : Span , }
};
}
