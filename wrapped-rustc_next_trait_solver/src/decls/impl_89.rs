macro_rules! deps {
    () => {
        SolverDelegate!();
        EvalCtxt!();
        ReplaceAliasWithInfer!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < 'me , 'a , D , I > ReplaceAliasWithInfer < 'me , 'a , D , I > where D : SolverDelegate < Interner = I > , I : Interner , { fn new (ecx : & 'me mut EvalCtxt < 'a , D > , for_goal_source : GoalSource , param_env : I :: ParamEnv ,) -> Self { let step_kind = ecx . step_kind_for_source (for_goal_source) ; ReplaceAliasWithInfer { ecx , param_env , normalization_goal_source : GoalSource :: NormalizeGoal (step_kind) , cache : Default :: default () , } } }
    };
}

impl_89!()