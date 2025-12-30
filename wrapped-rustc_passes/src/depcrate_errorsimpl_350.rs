// Generated macro for impl_350 (impl)
macro_rules! Depcrate_errorsimpl_350 {
() => {
// Module: crate::errors
// Provides: {"impl_350"}
// Dependencies: {}
impl Subdiagnostic for UnusedVariableStringInterp { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . span_label (self . lit , crate :: fluent_generated :: passes_maybe_string_interpolation) ; diag . multipart_suggestion (crate :: fluent_generated :: passes_string_interpolation_only_works , vec ! [(self . lo , String :: from ("format!(")) , (self . hi , String :: from (")"))] , Applicability :: MachineApplicable ,) ; } }
};
}
