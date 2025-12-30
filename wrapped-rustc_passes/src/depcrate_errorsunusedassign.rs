// Generated macro for UnusedAssign (struct)
macro_rules! Depcrate_errorsUnusedAssign {
() => {
// Module: crate::errors
// Provides: {"UnusedAssign"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (passes_unused_assign)] pub (crate) struct UnusedAssign { pub name : String , # [subdiagnostic] pub suggestion : Option < UnusedAssignSuggestion > , # [help] pub help : bool , }
};
}
