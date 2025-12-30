// Generated macro for impl_807 (impl)
macro_rules! Depcrate_lintsimpl_807 {
() => {
// Module: crate::lints
// Provides: {"impl_807"}
// Dependencies: {}
impl < 'a > LintDiagnostic < 'a , () > for DropTraitConstraintsDiag < '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (fluent :: lint_drop_trait_constraints) ; diag . arg ("predicate" , self . predicate) ; diag . arg ("needs_drop" , self . tcx . def_path_str (self . def_id)) ; } }
};
}
