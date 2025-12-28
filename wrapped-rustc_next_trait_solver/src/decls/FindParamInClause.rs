macro_rules! deps {
    () => {
        EvalCtxt!();
        SolverDelegate!();
    };
}

macro_rules! FindParamInClause {
    () => {
        deps!();
        struct FindParamInClause < 'a , 'b , D : SolverDelegate < Interner = I > , I : Interner > { ecx : & 'a mut EvalCtxt < 'b , D > , param_env : I :: ParamEnv , universes : Vec < Option < ty :: UniverseIndex > > , }
    };
}

FindParamInClause!()