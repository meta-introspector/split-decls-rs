macro_rules! type_op_ascribe_user_type {
    () => {
        fn type_op_ascribe_user_type < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , AscribeUserType < 'tcx > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , () > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , | ocx , key | { type_op_ascribe_user_type_with_span (ocx , key , DUMMY_SP) }) }
    };
}

type_op_ascribe_user_type!();