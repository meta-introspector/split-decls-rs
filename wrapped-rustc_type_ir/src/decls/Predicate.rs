macro_rules! deps {
    () => {
        ClauseKind!();
        Elaboratable!();
        UpcastFrom!();
        NormalizesTo!();
        Binder!();
        TraitRef!();
        Interner!();
        Flags!();
        TraitPredicate!();
        IntoKind!();
        TypeSuperFoldable!();
        PredicateKind!();
        Clause!();
        TypeSuperVisitable!();
        Ty!();
        Region!();
        OutlivesPredicate!();
    };
}

macro_rules! Predicate {
    () => {
        deps!();
        pub trait Predicate < I : Interner < Predicate = Self > > : Copy + Debug + Hash + Eq + TypeSuperVisitable < I > + TypeSuperFoldable < I > + Flags + UpcastFrom < I , ty :: PredicateKind < I > > + UpcastFrom < I , ty :: Binder < I , ty :: PredicateKind < I > > > + UpcastFrom < I , ty :: ClauseKind < I > > + UpcastFrom < I , ty :: Binder < I , ty :: ClauseKind < I > > > + UpcastFrom < I , I :: Clause > + UpcastFrom < I , ty :: NormalizesTo < I > > + UpcastFrom < I , ty :: TraitRef < I > > + UpcastFrom < I , ty :: Binder < I , ty :: TraitRef < I > > > + UpcastFrom < I , ty :: TraitPredicate < I > > + UpcastFrom < I , ty :: OutlivesPredicate < I , I :: Ty > > + UpcastFrom < I , ty :: OutlivesPredicate < I , I :: Region > > + IntoKind < Kind = ty :: Binder < I , ty :: PredicateKind < I > > > + Elaboratable < I > { fn as_clause (self) -> Option < I :: Clause > ; fn as_normalizes_to (self) -> Option < ty :: Binder < I , ty :: NormalizesTo < I > > > { let kind = self . kind () ; match kind . skip_binder () { ty :: PredicateKind :: NormalizesTo (pred) => Some (kind . rebind (pred)) , _ => None , } } fn allow_normalization (self) -> bool ; }
    };
}

Predicate!()