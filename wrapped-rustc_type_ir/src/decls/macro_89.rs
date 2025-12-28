macro_rules! deps {
    () => {
        ExistentialProjection!();
        SubtypePredicate!();
        HostEffectPredicate!();
        ExistentialTraitRef!();
        FnSig!();
        ProjectionPredicate!();
        TraitRef!();
        AliasTerm!();
        TraitPredicate!();
        CoercePredicate!();
        AliasTy!();
        NormalizesTo!();
        PatternKind!();
    };
}

macro_rules! macro_89 {
    () => {
        deps!();
        define_display_via_print ! (TraitRef , TraitPredicate , ExistentialTraitRef , ExistentialProjection , ProjectionPredicate , NormalizesTo , SubtypePredicate , CoercePredicate , HostEffectPredicate , AliasTy , AliasTerm , FnSig , PatternKind ,) ;
    };
}

macro_89!()