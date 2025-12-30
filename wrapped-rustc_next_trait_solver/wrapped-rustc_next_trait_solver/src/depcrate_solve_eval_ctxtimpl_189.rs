// Generated macro for impl_189 (impl)
macro_rules! Depcrate_solve_eval_ctxtimpl_189 {
() => {
// Module: crate::solve::eval_ctxt
// Provides: {"impl_189"}
// Dependencies: {}
impl < D , I > SolverDelegateEvalExt for D where D : SolverDelegate < Interner = I > , I : Interner , { # [instrument (level = "debug" , skip (self))] fn evaluate_root_goal (& self , goal : Goal < I , I :: Predicate > , span : I :: Span , stalled_on : Option < GoalStalledOn < I > > ,) -> Result < GoalEvaluation < I > , NoSolution > { EvalCtxt :: enter_root (self , self . cx () . recursion_limit () , span , | ecx | { ecx . evaluate_goal (GoalSource :: Misc , goal , stalled_on) }) } fn root_goal_may_hold_with_depth (& self , root_depth : usize , goal : Goal < Self :: Interner , < Self :: Interner as Interner > :: Predicate > ,) -> bool { self . probe (| | { EvalCtxt :: enter_root (self , root_depth , I :: Span :: dummy () , | ecx | { ecx . evaluate_goal (GoalSource :: Misc , goal , None) }) }) . is_ok () } # [instrument (level = "debug" , skip (self))] fn evaluate_root_goal_for_proof_tree (& self , goal : Goal < I , I :: Predicate > , span : I :: Span ,) -> (Result < NestedNormalizationGoals < I > , NoSolution > , inspect :: GoalEvaluation < I >) { evaluate_root_goal_for_proof_tree (self , goal , span) } }
};
}
