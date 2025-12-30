// Generated macro for UnexpectedCfgValue (struct)
macro_rules! Depcrate_lintsUnexpectedCfgValue {
() => {
// Module: crate::lints
// Provides: {"UnexpectedCfgValue"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_unexpected_cfg_value)] pub (crate) struct UnexpectedCfgValue { # [subdiagnostic] pub code_sugg : unexpected_cfg_value :: CodeSuggestion , # [subdiagnostic] pub invocation_help : unexpected_cfg_value :: InvocationHelp , pub has_value : bool , pub value : String , }
};
}
