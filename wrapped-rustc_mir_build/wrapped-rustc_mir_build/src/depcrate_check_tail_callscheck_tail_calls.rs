// Generated macro for check_tail_calls (function)
macro_rules! Depcrate_check_tail_callscheck_tail_calls {
() => {
// Module: crate::check_tail_calls
// Provides: {"check_tail_calls"}
// Dependencies: {}
pub (crate) fn check_tail_calls (tcx : TyCtxt < '_ > , def : LocalDefId) -> Result < () , ErrorGuaranteed > { let (thir , expr) = tcx . thir_body (def) ? ; let thir = & thir . borrow () ; if thir . exprs . is_empty () { return Ok (()) ; } let is_closure = matches ! (tcx . def_kind (def) , DefKind :: Closure) ; let caller_ty = tcx . type_of (def) . skip_binder () ; let mut visitor = TailCallCkVisitor { tcx , thir , found_errors : Ok (()) , typing_env : ty :: TypingEnv :: non_body_analysis (tcx , def) , is_closure , caller_ty , } ; visitor . visit_expr (& thir [expr]) ; visitor . found_errors }
};
}
