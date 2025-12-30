// Generated macro for CostChecker (struct)
macro_rules! Depcrate_cost_checkerCostChecker {
() => {
// Module: crate::cost_checker
// Provides: {"CostChecker"}
// Dependencies: {}
# [doc = " Verify that the callee body is compatible with the caller."] # [derive (Clone)] pub (super) struct CostChecker < 'b , 'tcx > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , penalty : usize , bonus : usize , callee_body : & 'b Body < 'tcx > , instance : Option < ty :: Instance < 'tcx > > , }
};
}
