macro_rules! deps {
    () => {
        FilterToTraits!();
        Clause!();
        Interner!();
        Elaborator!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < I : Interner > Elaborator < I , I :: Clause > { fn filter_to_traits (self) -> FilterToTraits < I , Self > { FilterToTraits { _cx : PhantomData , base_iterator : self } } }
    };
}

impl_27!()