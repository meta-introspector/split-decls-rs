// Generated macro for UnusedVarAssignedOnly (struct)
macro_rules! Depcrate_errorsUnusedVarAssignedOnly {
() => {
// Module: crate::errors
// Provides: {"UnusedVarAssignedOnly"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (passes_unused_var_assigned_only)] # [note] pub (crate) struct UnusedVarAssignedOnly { pub name : String , # [subdiagnostic] pub typo : Option < PatternTypo > , }
};
}
