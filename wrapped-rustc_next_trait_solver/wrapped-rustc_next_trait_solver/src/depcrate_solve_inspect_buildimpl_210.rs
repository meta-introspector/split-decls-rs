// Generated macro for impl_210 (impl)
macro_rules! Depcrate_solve_inspect_buildimpl_210 {
() => {
// Module: crate::solve::inspect::build
// Provides: {"impl_210"}
// Dependencies: {}
impl < I : Interner > WipEvaluationStep < I > { fn current_evaluation_scope (& mut self) -> & mut WipProbe < I > { let mut current = & mut self . evaluation ; for _ in 0 .. self . probe_depth { match current . steps . last_mut () { Some (WipProbeStep :: NestedProbe (p)) => current = p , _ => panic ! () , } } current } fn finalize (self) -> inspect :: Probe < I > { let evaluation = self . evaluation . finalize () ; match evaluation . kind { inspect :: ProbeKind :: Root { .. } => evaluation , _ => unreachable ! ("unexpected root evaluation: {evaluation:?}") , } } }
};
}
