// Generated macro for UnusedDuplicate (struct)
macro_rules! Depcrate_errorsUnusedDuplicate {
() => {
// Module: crate::errors
// Provides: {"UnusedDuplicate"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (passes_unused_duplicate)] pub (crate) struct UnusedDuplicate { # [suggestion (code = "" , applicability = "machine-applicable")] pub this : Span , # [note] pub other : Span , # [warning] pub warning : bool , }
};
}
