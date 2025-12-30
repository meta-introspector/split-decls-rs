// Generated macro for impl_720 (impl)
macro_rules! Depcrate_lintsimpl_720 {
() => {
// Module: crate::lints
// Provides: {"impl_720"}
// Dependencies: {}
impl Subdiagnostic for BuiltinClashingExternSub < '_ > { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { let mut expected_str = DiagStyledString :: new () ; expected_str . push (self . expected . fn_sig (self . tcx) . to_string () , false) ; let mut found_str = DiagStyledString :: new () ; found_str . push (self . found . fn_sig (self . tcx) . to_string () , true) ; diag . note_expected_found ("" , expected_str , "" , found_str) ; } }
};
}
