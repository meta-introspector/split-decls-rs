// Generated macro for impl_50 (impl)
macro_rules! Depcrate_errorsimpl_50 {
() => {
// Module: crate::errors
// Provides: {"impl_50"}
// Dependencies: {}
impl Subdiagnostic for Overlap { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { let Overlap { span , range } = self ; let message = format ! ("this range overlaps on `{range}`...") ; diag . span_label (span , message) ; } }
};
}
