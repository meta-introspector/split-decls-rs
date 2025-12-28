macro_rules! deps {
    () => {
        BoundConstness!();
        PredicatePolarity!();
        Variance!();
        GoalSource!();
        AliasRelationDirection!();
        BuiltinImplSource!();
        Certainty!();
    };
}

macro_rules! macro_214 {
    () => {
        deps!();
        TrivialTypeTraversalImpls ! { () , bool , usize , u16 , u32 , u64 , crate :: AliasRelationDirection , crate :: BoundConstness , crate :: DebruijnIndex , crate :: PredicatePolarity , crate :: UniverseIndex , crate :: Variance , crate :: solve :: BuiltinImplSource , crate :: solve :: Certainty , crate :: solve :: GoalSource , rustc_ast_ir :: Mutability , }
    };
}

macro_214!()