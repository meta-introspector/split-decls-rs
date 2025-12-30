// Generated macro for type_op_normalize_poly_fn_sig (function)
macro_rules! Depcrate_type_optype_op_normalize_poly_fn_sig {
() => {
// Module: crate::type_op
// Provides: {"type_op_normalize_poly_fn_sig"}
// Dependencies: {}
fn type_op_normalize_poly_fn_sig < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Normalize < PolyFnSig < 'tcx > > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , PolyFnSig < 'tcx > > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , type_op_normalize) }
};
}
