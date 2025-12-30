// Generated macro for UnusedVarRemoveField (struct)
macro_rules! Depcrate_errorsUnusedVarRemoveField {
() => {
// Module: crate::errors
// Provides: {"UnusedVarRemoveField"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (passes_unused_var_remove_field)] pub (crate) struct UnusedVarRemoveField { pub name : String , # [subdiagnostic] pub sugg : UnusedVarRemoveFieldSugg , }
};
}
