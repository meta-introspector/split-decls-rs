// Generated macro for normalize_canonicalized_inherent_projection_ty (function)
macro_rules! Depcrate_normalize_projection_tynormalize_canonicalized_inherent_projection_ty {
() => {
// Module: crate::normalize_projection_ty
// Provides: {"normalize_canonicalized_inherent_projection_ty"}
// Dependencies: {}
fn normalize_canonicalized_inherent_projection_ty < 'tcx > (tcx : TyCtxt < 'tcx > , goal : CanonicalAliasGoal < 'tcx > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , NormalizationResult < 'tcx > > > , NoSolution > { debug ! ("normalize_canonicalized_inherent_projection_ty(goal={:#?})" , goal) ; tcx . infer_ctxt () . enter_canonical_trait_query (& goal , | ocx , ParamEnvAnd { param_env , value : goal } | { let selcx = & mut SelectionContext :: new (ocx . infcx) ; let cause = ObligationCause :: dummy () ; let mut obligations = PredicateObligations :: new () ; let answer = traits :: normalize_inherent_projection (selcx , param_env , goal . into () , cause , 0 , & mut obligations ,) ; ocx . register_obligations (obligations) ; Ok (NormalizationResult { normalized_ty : answer . expect_type () }) } ,) }
};
}
