macro_rules! deps {
    () => {
        TraitPredicate!();
        Interner!();
        TraitRef!();
        PredicatePolarity!();
        Binder!();
        UpcastFrom!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl < I : Interner > UpcastFrom < I , ty :: Binder < I , TraitRef < I > > > for ty :: Binder < I , TraitPredicate < I > > { fn upcast_from (from : ty :: Binder < I , TraitRef < I > > , _tcx : I) -> Self { from . map_bound (| trait_ref | TraitPredicate { trait_ref , polarity : PredicatePolarity :: Positive , }) } }
    };
}

impl_352!();