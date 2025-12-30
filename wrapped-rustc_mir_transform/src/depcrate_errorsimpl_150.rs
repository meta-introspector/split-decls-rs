// Generated macro for impl_150 (impl)
macro_rules! Depcrate_errorsimpl_150 {
() => {
// Module: crate::errors
// Provides: {"impl_150"}
// Dependencies: {}
impl < 'a > LintDiagnostic < 'a , () > for MustNotSupend < '_ , '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut rustc_errors :: Diag < 'a , () >) { diag . primary_message (fluent :: mir_transform_must_not_suspend) ; diag . span_label (self . yield_sp , fluent :: _subdiag :: label) ; if let Some (reason) = self . reason { diag . subdiagnostic (reason) ; } diag . span_help (self . src_sp , fluent :: _subdiag :: help) ; diag . arg ("pre" , self . pre) ; diag . arg ("def_path" , self . tcx . def_path_str (self . def_id)) ; diag . arg ("post" , self . post) ; } }
};
}
