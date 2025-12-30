// Generated macro for impl_503 (impl)
macro_rules! Depcrate_errorsimpl_503 {
() => {
// Module: crate::errors
// Provides: {"impl_503"}
// Dependencies: {}
impl Subdiagnostic for UnsafeNotInheritedLintNote { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . span_note (self . signature_span , fluent :: mir_build_unsafe_fn_safe_body) ; let body_start = self . body_span . shrink_to_lo () ; let body_end = self . body_span . shrink_to_hi () ; diag . tool_only_multipart_suggestion (fluent :: mir_build_wrap_suggestion , vec ! [(body_start , "{ unsafe " . into ()) , (body_end , "}" . into ())] , Applicability :: MachineApplicable ,) ; } }
};
}
