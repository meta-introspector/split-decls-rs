macro_rules! try_normalize_after_erasing_regions {
    () => {
        fn try_normalize_after_erasing_regions < 'tcx , T : TypeFoldable < TyCtxt < 'tcx > > + PartialEq + Copy > (tcx : TyCtxt < 'tcx > , goal : PseudoCanonicalInput < 'tcx , T > ,) -> Result < T , NoSolution > { let PseudoCanonicalInput { typing_env , value } = goal ; let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; let cause = ObligationCause :: dummy () ; match infcx . at (& cause , param_env) . query_normalize (value) { Ok (Normalized { value : normalized_value , obligations : normalized_obligations }) => { assert_eq ! (normalized_obligations . iter () . find (| p | not_outlives_predicate (p . predicate)) , None ,) ; let resolved_value = infcx . resolve_vars_if_possible (normalized_value) ; debug_assert_eq ! (normalized_value , resolved_value) ; let erased = infcx . tcx . erase_and_anonymize_regions (resolved_value) ; debug_assert ! (! erased . has_infer () , "{erased:?}") ; Ok (erased) } Err (NoSolution) => Err (NoSolution) , } }
    };
}

try_normalize_after_erasing_regions!()