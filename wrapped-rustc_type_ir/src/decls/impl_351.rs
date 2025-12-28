macro_rules! deps {
    () => {
        TraitRef!();
        UpcastFrom!();
        Interner!();
        TraitPredicate!();
        PredicatePolarity!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl < I : Interner > UpcastFrom < I , TraitRef < I > > for TraitPredicate < I > { fn upcast_from (from : TraitRef < I > , _tcx : I) -> Self { TraitPredicate { trait_ref : from , polarity : PredicatePolarity :: Positive } } }
    };
}

impl_351!();