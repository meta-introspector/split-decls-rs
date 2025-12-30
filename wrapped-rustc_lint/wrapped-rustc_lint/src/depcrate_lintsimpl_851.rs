// Generated macro for impl_851 (impl)
macro_rules! Depcrate_lintsimpl_851 {
() => {
// Module: crate::lints
// Provides: {"impl_851"}
// Dependencies: {}
impl < 'a > LintDiagnostic < 'a , () > for UnusedDef < '_ , '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (fluent :: lint_unused_def) ; diag . arg ("pre" , self . pre) ; diag . arg ("post" , self . post) ; diag . arg ("def" , self . cx . tcx . def_path_str (self . def_id)) ; if let Some (note) = self . note { diag . note (note . to_string ()) ; } if let Some (sugg) = self . suggestion { diag . subdiagnostic (sugg) ; } } }
};
}
