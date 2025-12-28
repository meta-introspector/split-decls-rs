macro_rules! normalize_canonicalized_free_alias {
    () => {
        fn normalize_canonicalized_free_alias < 'tcx > (tcx : TyCtxt < 'tcx > , goal : CanonicalAliasGoal < 'tcx > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , NormalizationResult < 'tcx > > > , NoSolution > { debug ! ("normalize_canonicalized_free_alias(goal={:#?})" , goal) ; tcx . infer_ctxt () . enter_canonical_trait_query (& goal , | ocx , ParamEnvAnd { param_env , value : goal } | { let obligations = tcx . predicates_of (goal . def_id) . instantiate_own (tcx , goal . args) . map (| (predicate , span) | { traits :: Obligation :: new (tcx , ObligationCause :: dummy_with_span (span) , param_env , predicate ,) } ,) ; ocx . register_obligations (obligations) ; let normalized_ty = tcx . type_of (goal . def_id) . instantiate (tcx , goal . args) ; Ok (NormalizationResult { normalized_ty }) } ,) }
    };
}

normalize_canonicalized_free_alias!()