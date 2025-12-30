// Generated macro for evaluate_obligation (function)
macro_rules! Depcrate_evaluate_obligationevaluate_obligation {
() => {
// Module: crate::evaluate_obligation
// Provides: {"evaluate_obligation"}
// Dependencies: {}
fn evaluate_obligation < 'tcx > (tcx : TyCtxt < 'tcx > , canonical_goal : CanonicalPredicateGoal < 'tcx > ,) -> Result < EvaluationResult , OverflowError > { assert ! (! tcx . next_trait_solver_globally ()) ; debug ! ("evaluate_obligation(canonical_goal={:#?})" , canonical_goal) ; let (ref infcx , goal , _var_values) = tcx . infer_ctxt () . build_with_canonical (DUMMY_SP , & canonical_goal) ; debug ! ("evaluate_obligation: goal={:#?}" , goal) ; let ParamEnvAnd { param_env , value : predicate } = goal ; if sizedness_fast_path (tcx , predicate , param_env) { return Ok (EvaluationResult :: EvaluatedToOk) ; } let mut selcx = SelectionContext :: with_query_mode (infcx , TraitQueryMode :: Canonical) ; let obligation = Obligation :: new (tcx , ObligationCause :: dummy () , param_env , predicate) ; selcx . evaluate_root_obligation (& obligation) }
};
}
