// Generated macro for check_feature_dependent_abi (function)
macro_rules! Depcrate_mono_checks_abi_checkcheck_feature_dependent_abi {
() => {
// Module: crate::mono_checks::abi_check
// Provides: {"check_feature_dependent_abi"}
// Dependencies: {}
pub (crate) fn check_feature_dependent_abi < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , body : & 'tcx mir :: Body < 'tcx > ,) { check_instance_abi (tcx , instance) ; check_callees_abi (tcx , instance , body) ; }
};
}
