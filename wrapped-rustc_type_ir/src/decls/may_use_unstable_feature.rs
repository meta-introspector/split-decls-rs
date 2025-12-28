macro_rules! deps {
    () => {
        TypingMode!();
        Interner!();
        InferCtxtLike!();
        ClauseKind!();
        ParamEnv!();
    };
}

macro_rules! may_use_unstable_feature {
    () => {
        deps!();
        pub fn may_use_unstable_feature < 'a , I : Interner , Infcx > (infcx : & 'a Infcx , param_env : I :: ParamEnv , symbol : I :: Symbol ,) -> bool where Infcx : InferCtxtLike < Interner = I > , { for pred in param_env . caller_bounds () . iter () { if let ty :: ClauseKind :: UnstableFeature (sym) = pred . kind () . skip_binder () { if sym == symbol { return true ; } } } (infcx . typing_mode () == TypingMode :: PostAnalysis) || infcx . cx () . features () . feature_bound_holds_in_crate (symbol) }
    };
}

may_use_unstable_feature!();