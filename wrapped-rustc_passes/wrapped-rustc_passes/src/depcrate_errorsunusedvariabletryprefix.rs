// Generated macro for UnusedVariableTryPrefix (struct)
macro_rules! Depcrate_errorsUnusedVariableTryPrefix {
() => {
// Module: crate::errors
// Provides: {"UnusedVariableTryPrefix"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (passes_unused_variable_try_prefix)] pub (crate) struct UnusedVariableTryPrefix { # [label] pub label : Option < Span > , # [subdiagnostic] pub string_interp : Vec < UnusedVariableStringInterp > , # [subdiagnostic] pub sugg : UnusedVariableSugg , pub name : String , # [subdiagnostic] pub typo : Option < PatternTypo > , }
};
}
