macro_rules! type_op_prove_predicate {
    () => {
        fn type_op_prove_predicate < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , ProvePredicate < 'tcx > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , () > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , | ocx , key | { type_op_prove_predicate_with_cause (ocx , key , ObligationCause :: dummy ()) ; Ok (()) }) }
    };
}

type_op_prove_predicate!()