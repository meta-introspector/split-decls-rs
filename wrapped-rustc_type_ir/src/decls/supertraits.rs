macro_rules! deps {
    () => {
        Binder!();
        FilterToTraits!();
        Elaborator!();
        Clause!();
        TraitRef!();
        Interner!();
    };
}

macro_rules! supertraits {
    () => {
        deps!();
        pub fn supertraits < I : Interner > (cx : I , trait_ref : ty :: Binder < I , ty :: TraitRef < I > > ,) -> FilterToTraits < I , Elaborator < I , I :: Clause > > { elaborate (cx , [trait_ref . upcast (cx)]) . filter_only_self () . filter_to_traits () }
    };
}

supertraits!();