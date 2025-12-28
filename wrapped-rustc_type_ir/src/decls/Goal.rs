macro_rules! deps {
    () => {
        Interner!();
        ParamEnv!();
    };
}

macro_rules! Goal {
    () => {
        deps!();
        # [doc = " A goal is a statement, i.e. `predicate`, we want to prove"] # [doc = " given some assumptions, i.e. `param_env`."] # [doc = ""] # [doc = " Most of the time the `param_env` contains the `where`-bounds of the function"] # [doc = " we're currently typechecking while the `predicate` is some trait bound."] # [derive_where (Clone , Hash , PartialEq , Debug ; I : Interner , P)] # [derive_where (Copy ; I : Interner , P : Copy)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct Goal < I : Interner , P > { pub param_env : I :: ParamEnv , pub predicate : P , }
    };
}

Goal!();