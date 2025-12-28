macro_rules! deps {
    () => {
        Interner!();
        FilterToTraits!();
        Clause!();
        TraitRef!();
        Binder!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < I : Interner , It : Iterator < Item = I :: Clause > > Iterator for FilterToTraits < I , It > { type Item = ty :: Binder < I , ty :: TraitRef < I > > ; fn next (& mut self) -> Option < ty :: Binder < I , ty :: TraitRef < I > > > { while let Some (pred) = self . base_iterator . next () { if let Some (data) = pred . as_trait_clause () { return Some (data . map_bound (| t | t . trait_ref)) ; } } None } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . base_iterator . size_hint () ; (0 , upper) } }
    };
}

impl_29!();