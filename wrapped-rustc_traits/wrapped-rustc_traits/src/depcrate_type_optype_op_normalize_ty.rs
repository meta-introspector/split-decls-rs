// Generated macro for type_op_normalize_ty (function)
macro_rules! Depcrate_type_optype_op_normalize_ty {
() => {
// Module: crate::type_op
// Provides: {"type_op_normalize_ty"}
// Dependencies: {}
fn type_op_normalize_ty < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Normalize < Ty < 'tcx > > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , Ty < 'tcx > > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , type_op_normalize) }
};
}
