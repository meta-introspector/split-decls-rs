// Generated macro for implied_outlives_bounds (function)
macro_rules! Depcrate_implied_outlives_boundsimplied_outlives_bounds {
() => {
// Module: crate::implied_outlives_bounds
// Provides: {"implied_outlives_bounds"}
// Dependencies: {}
fn implied_outlives_bounds < 'tcx > (tcx : TyCtxt < 'tcx > , (goal , disable_implied_bounds_hack) : (CanonicalImpliedOutlivesBoundsGoal < 'tcx > , bool) ,) -> Result < & 'tcx Canonical < 'tcx , canonical :: QueryResponse < 'tcx , Vec < OutlivesBound < 'tcx > > > > , NoSolution , > { tcx . infer_ctxt () . enter_canonical_trait_query (& goal , | ocx , key | { let ParamEnvAnd { param_env , value : ImpliedOutlivesBounds { ty } } = key ; compute_implied_outlives_bounds_inner (ocx , param_env , ty , DUMMY_SP , disable_implied_bounds_hack ,) }) }
};
}
