// Generated macro for validate_body (function)
macro_rules! Depcrate_pass_managervalidate_body {
() => {
// Module: crate::pass_manager
// Provides: {"validate_body"}
// Dependencies: {}
pub (super) fn validate_body < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx > , when : String) { validate :: Validator { when } . run_pass (tcx , body) ; }
};
}
