macro_rules! deps {
    () => {
        Relate!();
        SliceLike!();
        ExistentialProjection!();
        ExistentialPredicate!();
        Interner!();
        Binder!();
        ExistentialTraitRef!();
    };
}

macro_rules! BoundExistentialPredicates {
    () => {
        deps!();
        pub trait BoundExistentialPredicates < I : Interner > : Copy + Debug + Hash + Eq + Relate < I > + SliceLike < Item = ty :: Binder < I , ty :: ExistentialPredicate < I > > > { fn principal_def_id (self) -> Option < I :: TraitId > ; fn principal (self) -> Option < ty :: Binder < I , ty :: ExistentialTraitRef < I > > > ; fn auto_traits (self) -> impl IntoIterator < Item = I :: TraitId > ; fn projection_bounds (self ,) -> impl IntoIterator < Item = ty :: Binder < I , ty :: ExistentialProjection < I > > > ; }
    };
}

BoundExistentialPredicates!();