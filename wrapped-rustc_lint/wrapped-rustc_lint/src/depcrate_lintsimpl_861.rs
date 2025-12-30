// Generated macro for impl_861 (impl)
macro_rules! Depcrate_lintsimpl_861 {
() => {
// Module: crate::lints
// Provides: {"impl_861"}
// Dependencies: {}
impl < 'a > LintDiagnostic < 'a , () > for AsyncFnInTraitDiag { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (fluent :: lint_async_fn_in_trait) ; diag . note (fluent :: lint_note) ; if let Some (sugg) = self . sugg { diag . multipart_suggestion (fluent :: lint_suggestion , sugg , Applicability :: MaybeIncorrect) ; } } }
};
}
