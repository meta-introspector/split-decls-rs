// Generated macro for type_op_normalize_clause (function)
macro_rules! Depcrate_type_optype_op_normalize_clause {
() => {
// Module: crate::type_op
// Provides: {"type_op_normalize_clause"}
// Dependencies: {}
fn type_op_normalize_clause < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Normalize < Clause < 'tcx > > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , Clause < 'tcx > > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , type_op_normalize) }
};
}
