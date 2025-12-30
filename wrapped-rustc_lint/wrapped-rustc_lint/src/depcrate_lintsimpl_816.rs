// Generated macro for impl_816 (impl)
macro_rules! Depcrate_lintsimpl_816 {
() => {
// Module: crate::lints
// Provides: {"impl_816"}
// Dependencies: {}
impl Subdiagnostic for OverflowingBinHexSign { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { match self { OverflowingBinHexSign :: Positive => { diag . note (fluent :: lint_positive_note) ; } OverflowingBinHexSign :: Negative => { diag . note (fluent :: lint_negative_note) ; diag . note (fluent :: lint_negative_becomes_note) ; } } } }
};
}
