// Generated macro for compute_assumptions (function)
macro_rules! Depcrate_coroutine_witnessescompute_assumptions {
() => {
// Module: crate::coroutine_witnesses
// Provides: {"compute_assumptions"}
// Dependencies: {}
fn compute_assumptions < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , bound_tys : & 'tcx ty :: List < Ty < 'tcx > > ,) -> & 'tcx ty :: List < ty :: ArgOutlivesPredicate < 'tcx > > { let infcx = tcx . infer_ctxt () . build (ty :: TypingMode :: Analysis { defining_opaque_types_and_generators : ty :: List :: empty () , }) ; with_replaced_escaping_bound_vars (& infcx , & mut vec ! [None] , bound_tys , | bound_tys | { let param_env = tcx . param_env (def_id) ; let ocx = ObligationCtxt :: new (& infcx) ; ocx . register_obligations (bound_tys . iter () . map (| ty | { Obligation :: new (tcx , ObligationCause :: dummy () , param_env , ty :: ClauseKind :: WellFormed (ty . into ()) ,) })) ; let _errors = ocx . select_all_or_error () ; let region_obligations = infcx . take_registered_region_obligations () ; let region_assumptions = infcx . take_registered_region_assumptions () ; let region_constraints = infcx . take_and_reset_region_constraints () ; let outlives = make_query_region_constraints (region_obligations , & region_constraints , region_assumptions ,) . outlives . fold_with (& mut OpportunisticRegionResolver :: new (& infcx)) ; tcx . mk_outlives_from_iter (outlives . into_iter () . map (| (o , _) | o) . filter (| o | ! o . has_infer ()) ,) }) }
};
}
