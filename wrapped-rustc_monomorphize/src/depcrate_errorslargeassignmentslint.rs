// Generated macro for LargeAssignmentsLint (struct)
macro_rules! Depcrate_errorsLargeAssignmentsLint {
() => {
// Module: crate::errors
// Provides: {"LargeAssignmentsLint"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (monomorphize_large_assignments)] # [note] pub (crate) struct LargeAssignmentsLint { # [label] pub span : Span , pub size : u64 , pub limit : u64 , }
};
}
