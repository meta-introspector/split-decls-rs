macro_rules! deps {
    () => {
        ExistentialTraitRef!();
        TraitRef!();
        HostEffectPredicate!();
        TraitPredicate!();
        ProjectionPredicate!();
        ExistentialProjection!();
        NormalizesTo!();
        SubtypePredicate!();
        CoercePredicate!();
        AliasTerm!();
        FnSig!();
        AliasTy!();
        PatternKind!();
    };
}

macro_rules! macro_89 {
    () => {
        deps!();
        define_display_via_print ! (TraitRef , TraitPredicate , ExistentialTraitRef , ExistentialProjection , ProjectionPredicate , NormalizesTo , SubtypePredicate , CoercePredicate , HostEffectPredicate , AliasTy , AliasTerm , FnSig , PatternKind ,) ;
    };
}

macro_89!();