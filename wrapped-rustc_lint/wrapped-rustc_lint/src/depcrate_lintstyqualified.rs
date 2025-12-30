// Generated macro for TyQualified (struct)
macro_rules! Depcrate_lintsTyQualified {
() => {
// Module: crate::lints
// Provides: {"TyQualified"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_ty_qualified)] pub (crate) struct TyQualified { pub ty : String , # [suggestion (code = "{ty}" , applicability = "maybe-incorrect")] pub suggestion : Span , }
};
}
