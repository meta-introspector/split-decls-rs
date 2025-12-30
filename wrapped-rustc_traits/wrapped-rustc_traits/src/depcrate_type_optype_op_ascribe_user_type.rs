// Generated macro for type_op_ascribe_user_type (function)
macro_rules! Depcrate_type_optype_op_ascribe_user_type {
() => {
// Module: crate::type_op
// Provides: {"type_op_ascribe_user_type"}
// Dependencies: {}
fn type_op_ascribe_user_type < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , AscribeUserType < 'tcx > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , () > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , | ocx , key | { type_op_ascribe_user_type_with_span (ocx , key , DUMMY_SP) }) }
};
}
