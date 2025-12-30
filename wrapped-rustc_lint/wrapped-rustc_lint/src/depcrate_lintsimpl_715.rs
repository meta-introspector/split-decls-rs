// Generated macro for impl_715 (impl)
macro_rules! Depcrate_lintsimpl_715 {
() => {
// Module: crate::lints
// Provides: {"impl_715"}
// Dependencies: {}
impl < 'a > LintDiagnostic < 'a , () > for BuiltinUnpermittedTypeInit < '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (self . msg) ; diag . arg ("ty" , self . ty) ; diag . span_label (self . label , fluent :: lint_builtin_unpermitted_type_init_label) ; if let InhabitedPredicate :: True = self . ty . inhabited_predicate (self . tcx) { diag . span_label (self . label , fluent :: lint_builtin_unpermitted_type_init_label_suggestion ,) ; } self . sub . add_to_diag (diag) ; } }
};
}
