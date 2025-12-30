// Generated macro for impl_933 (impl)
macro_rules! Depcrate_lintsimpl_933 {
() => {
// Module: crate::lints
// Provides: {"impl_933"}
// Dependencies: {}
impl < 'a , G : EmissionGuarantee > LintDiagnostic < 'a , G > for AmbiguousGlobImports { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , G >) { diag . primary_message (self . ambiguity . msg . clone ()) ; rustc_errors :: report_ambiguity_error (diag , self . ambiguity) ; } }
};
}
