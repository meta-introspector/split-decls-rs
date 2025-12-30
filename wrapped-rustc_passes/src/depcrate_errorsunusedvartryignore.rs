// Generated macro for UnusedVarTryIgnore (struct)
macro_rules! Depcrate_errorsUnusedVarTryIgnore {
() => {
// Module: crate::errors
// Provides: {"UnusedVarTryIgnore"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (passes_unused_variable_try_ignore)] pub (crate) struct UnusedVarTryIgnore { pub name : String , # [subdiagnostic] pub sugg : UnusedVarTryIgnoreSugg , }
};
}
