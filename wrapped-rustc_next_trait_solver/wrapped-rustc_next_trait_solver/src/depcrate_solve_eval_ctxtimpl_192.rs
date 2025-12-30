// Generated macro for impl_192 (impl)
macro_rules! Depcrate_solve_eval_ctxtimpl_192 {
() => {
// Module: crate::solve::eval_ctxt
// Provides: {"impl_192"}
// Dependencies: {}
impl < 'me , 'a , D , I > ReplaceAliasWithInfer < 'me , 'a , D , I > where D : SolverDelegate < Interner = I > , I : Interner , { fn new (ecx : & 'me mut EvalCtxt < 'a , D > , for_goal_source : GoalSource , param_env : I :: ParamEnv ,) -> Self { let step_kind = ecx . step_kind_for_source (for_goal_source) ; ReplaceAliasWithInfer { ecx , param_env , normalization_goal_source : GoalSource :: NormalizeGoal (step_kind) , cache : Default :: default () , } } }
};
}
