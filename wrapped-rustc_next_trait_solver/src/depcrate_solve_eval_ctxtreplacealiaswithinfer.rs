// Generated macro for ReplaceAliasWithInfer (struct)
macro_rules! Depcrate_solve_eval_ctxtReplaceAliasWithInfer {
() => {
// Module: crate::solve::eval_ctxt
// Provides: {"ReplaceAliasWithInfer"}
// Dependencies: {}
# [doc = " Eagerly replace aliases with inference variables, emitting `AliasRelate`"] # [doc = " goals, used when adding goals to the `EvalCtxt`. We compute the"] # [doc = " `AliasRelate` goals before evaluating the actual goal to get all the"] # [doc = " constraints we can."] # [doc = ""] # [doc = " This is a performance optimization to more eagerly detect cycles during trait"] # [doc = " solving. See tests/ui/traits/next-solver/cycles/cycle-modulo-ambig-aliases.rs."] # [doc = ""] # [doc = " The emitted goals get evaluated in the context of the parent goal; by"] # [doc = " replacing aliases in nested goals we essentially pull the normalization out of"] # [doc = " the nested goal. We want to treat the goal as if the normalization still happens"] # [doc = " inside of the nested goal by inheriting the `step_kind` of the nested goal and"] # [doc = " storing it in the `GoalSource` of the emitted `AliasRelate` goals."] # [doc = " This is necessary for tests/ui/sized/coinductive-1.rs to compile."] struct ReplaceAliasWithInfer < 'me , 'a , D , I > where D : SolverDelegate < Interner = I > , I : Interner , { ecx : & 'me mut EvalCtxt < 'a , D > , param_env : I :: ParamEnv , normalization_goal_source : GoalSource , cache : HashMap < I :: Ty , I :: Ty > , }
};
}
