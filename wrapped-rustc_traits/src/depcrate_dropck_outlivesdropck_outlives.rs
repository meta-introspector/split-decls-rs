// Generated macro for dropck_outlives (function)
macro_rules! Depcrate_dropck_outlivesdropck_outlives {
() => {
// Module: crate::dropck_outlives
// Provides: {"dropck_outlives"}
// Dependencies: {}
fn dropck_outlives < 'tcx > (tcx : TyCtxt < 'tcx > , canonical_goal : CanonicalDropckOutlivesGoal < 'tcx > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , DropckOutlivesResult < 'tcx > > > , NoSolution > { debug ! ("dropck_outlives(goal={:#?})" , canonical_goal) ; tcx . infer_ctxt () . enter_canonical_trait_query (& canonical_goal , | ocx , goal | { compute_dropck_outlives_inner (ocx , goal , DUMMY_SP) }) }
};
}
