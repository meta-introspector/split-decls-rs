macro_rules! deps {
    () => {
        SolverDelegate!();
        EvalCtxt!();
    };
}

macro_rules! ReplaceProjectionWith {
    () => {
        deps!();
        struct ReplaceProjectionWith < 'a , 'b , I : Interner , D : SolverDelegate < Interner = I > > { ecx : & 'a mut EvalCtxt < 'b , D > , param_env : I :: ParamEnv , self_ty : I :: Ty , mapping : & 'a HashMap < I :: DefId , Vec < ty :: Binder < I , ty :: ProjectionPredicate < I > > > > , nested : Vec < Goal < I , I :: Predicate > > , }
    };
}

ReplaceProjectionWith!();