// Generated macro for AmbiguousNegativeLiteralsDiag (struct)
macro_rules! Depcrate_lintsAmbiguousNegativeLiteralsDiag {
() => {
// Module: crate::lints
// Provides: {"AmbiguousNegativeLiteralsDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_ambiguous_negative_literals)] # [note (lint_example)] pub (crate) struct AmbiguousNegativeLiteralsDiag { # [subdiagnostic] pub negative_literal : AmbiguousNegativeLiteralsNegativeLiteralSuggestion , # [subdiagnostic] pub current_behavior : AmbiguousNegativeLiteralsCurrentBehaviorSuggestion , }
};
}
