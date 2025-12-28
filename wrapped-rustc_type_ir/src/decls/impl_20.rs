macro_rules! deps {
    () => {
        Predicate!();
        Elaboratable!();
        TraitPredicate!();
        Clause!();
        ClauseWithSupertraitSpan!();
        Interner!();
        Span!();
        Binder!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < I : Interner > Elaboratable < I > for ClauseWithSupertraitSpan < I > { fn predicate (& self) -> < I as Interner > :: Predicate { self . clause . as_predicate () } fn child (& self , clause : < I as Interner > :: Clause) -> Self { ClauseWithSupertraitSpan { clause , supertrait_span : self . supertrait_span } } fn child_with_derived_cause (& self , clause : < I as Interner > :: Clause , supertrait_span : < I as Interner > :: Span , _parent_trait_pred : crate :: Binder < I , crate :: TraitPredicate < I > > , _index : usize ,) -> Self { ClauseWithSupertraitSpan { clause , supertrait_span } } }
    };
}

impl_20!()