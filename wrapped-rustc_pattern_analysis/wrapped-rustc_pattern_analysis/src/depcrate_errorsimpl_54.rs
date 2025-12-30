// Generated macro for impl_54 (impl)
macro_rules! Depcrate_errorsimpl_54 {
() => {
// Module: crate::errors
// Provides: {"impl_54"}
// Dependencies: {}
impl Subdiagnostic for GappedRange { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { let GappedRange { span , gap , first_range } = self ; let message = format ! ("this could appear to continue range `{first_range}`, but `{gap}` isn't matched by \
            either of them") ; diag . span_label (span , message) ; } }
};
}
