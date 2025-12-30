// Generated macro for impl_545 (impl)
macro_rules! Depcrate_errorsimpl_545 {
() => {
// Module: crate::errors
// Provides: {"impl_545"}
// Dependencies: {}
impl < 'tcx > Subdiagnostic for AdtDefinedHere < 'tcx > { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . arg ("ty" , self . ty) ; let mut spans = MultiSpan :: from (self . adt_def_span) ; for Variant { span } in self . variants { spans . push_span_label (span , fluent :: mir_build_variant_defined_here) ; } diag . span_note (spans , fluent :: mir_build_adt_defined_here) ; } }
};
}
