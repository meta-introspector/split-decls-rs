// Generated macro for AmbiguousGlobReexports (struct)
macro_rules! Depcrate_lintsAmbiguousGlobReexports {
() => {
// Module: crate::lints
// Provides: {"AmbiguousGlobReexports"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_ambiguous_glob_reexport)] pub (crate) struct AmbiguousGlobReexports { # [label (lint_label_first_reexport)] pub first_reexport : Span , # [label (lint_label_duplicate_reexport)] pub duplicate_reexport : Span , pub name : String , pub namespace : String , }
};
}
