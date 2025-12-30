// Generated macro for RedundantSemicolonsDiag (struct)
macro_rules! Depcrate_lintsRedundantSemicolonsDiag {
() => {
// Module: crate::lints
// Provides: {"RedundantSemicolonsDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_redundant_semicolons)] pub (crate) struct RedundantSemicolonsDiag { pub multiple : bool , # [subdiagnostic] pub suggestion : Option < RedundantSemicolonsSuggestion > , }
};
}
