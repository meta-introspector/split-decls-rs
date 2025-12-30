// Generated macro for BuiltinClashingExtern (enum)
macro_rules! Depcrate_lintsBuiltinClashingExtern {
() => {
// Module: crate::lints
// Provides: {"BuiltinClashingExtern"}
// Dependencies: {}
# [derive (LintDiagnostic)] pub (crate) enum BuiltinClashingExtern < 'a > { # [diag (lint_builtin_clashing_extern_same_name)] SameName { this : Symbol , orig : Symbol , # [label (lint_previous_decl_label)] previous_decl_label : Span , # [label (lint_mismatch_label)] mismatch_label : Span , # [subdiagnostic] sub : BuiltinClashingExternSub < 'a > , } , # [diag (lint_builtin_clashing_extern_diff_name)] DiffName { this : Symbol , orig : Symbol , # [label (lint_previous_decl_label)] previous_decl_label : Span , # [label (lint_mismatch_label)] mismatch_label : Span , # [subdiagnostic] sub : BuiltinClashingExternSub < 'a > , } , }
};
}
