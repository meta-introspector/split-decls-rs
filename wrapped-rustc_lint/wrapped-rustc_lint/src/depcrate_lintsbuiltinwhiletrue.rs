// Generated macro for BuiltinWhileTrue (struct)
macro_rules! Depcrate_lintsBuiltinWhileTrue {
() => {
// Module: crate::lints
// Provides: {"BuiltinWhileTrue"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_builtin_while_true)] pub (crate) struct BuiltinWhileTrue { # [suggestion (style = "short" , code = "{replace}" , applicability = "machine-applicable")] pub suggestion : Span , pub replace : String , }
};
}
