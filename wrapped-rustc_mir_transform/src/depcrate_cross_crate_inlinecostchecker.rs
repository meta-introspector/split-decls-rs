// Generated macro for CostChecker (struct)
macro_rules! Depcrate_cross_crate_inlineCostChecker {
() => {
// Module: crate::cross_crate_inline
// Provides: {"CostChecker"}
// Dependencies: {}
struct CostChecker < 'b , 'tcx > { tcx : TyCtxt < 'tcx > , callee_body : & 'b Body < 'tcx > , calls : usize , statements : usize , landing_pads : usize , resumes : usize , }
};
}
