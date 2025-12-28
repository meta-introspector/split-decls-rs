macro_rules! type_op_normalize_poly_fn_sig {
    () => {
        fn type_op_normalize_poly_fn_sig < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Normalize < PolyFnSig < 'tcx > > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , PolyFnSig < 'tcx > > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , type_op_normalize) }
    };
}

type_op_normalize_poly_fn_sig!();