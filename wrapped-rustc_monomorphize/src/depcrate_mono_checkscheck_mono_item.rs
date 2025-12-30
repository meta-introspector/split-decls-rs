// Generated macro for check_mono_item (function)
macro_rules! Depcrate_mono_checkscheck_mono_item {
() => {
// Module: crate::mono_checks
// Provides: {"check_mono_item"}
// Dependencies: {}
fn check_mono_item < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx >) { let body = tcx . instance_mir (instance . def) ; abi_check :: check_feature_dependent_abi (tcx , instance , body) ; move_check :: check_moves (tcx , instance , body) ; }
};
}
