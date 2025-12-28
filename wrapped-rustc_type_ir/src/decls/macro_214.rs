macro_rules! deps {
    () => {
        AliasRelationDirection!();
        BoundConstness!();
        PredicatePolarity!();
        BuiltinImplSource!();
        Variance!();
        Certainty!();
        GoalSource!();
    };
}

macro_rules! macro_214 {
    () => {
        deps!();
        TrivialTypeTraversalImpls ! { () , bool , usize , u16 , u32 , u64 , crate :: AliasRelationDirection , crate :: BoundConstness , crate :: DebruijnIndex , crate :: PredicatePolarity , crate :: UniverseIndex , crate :: Variance , crate :: solve :: BuiltinImplSource , crate :: solve :: Certainty , crate :: solve :: GoalSource , rustc_ast_ir :: Mutability , }
    };
}

macro_214!();