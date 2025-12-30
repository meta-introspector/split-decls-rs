// Generated macro for type_op_prove_predicate (function)
macro_rules! Depcrate_type_optype_op_prove_predicate {
() => {
// Module: crate::type_op
// Provides: {"type_op_prove_predicate"}
// Dependencies: {}
fn type_op_prove_predicate < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , ProvePredicate < 'tcx > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , () > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , | ocx , key | { type_op_prove_predicate_with_cause (ocx , key , ObligationCause :: dummy ()) ; Ok (()) }) }
};
}
