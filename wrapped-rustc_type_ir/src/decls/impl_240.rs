macro_rules! deps {
    () => {
        Interner!();
        IterInstantiated!();
        SliceLike!();
        GenericArg!();
        TypeFoldable!();
        EarlyBinder!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl < I : Interner , Iter : IntoIterator , A > Iterator for IterInstantiated < I , Iter , A > where Iter :: Item : TypeFoldable < I > , A : SliceLike < Item = I :: GenericArg > , { type Item = Iter :: Item ; fn next (& mut self) -> Option < Self :: Item > { Some (EarlyBinder { value : self . it . next () ? , _tcx : PhantomData } . instantiate (self . cx , self . args) ,) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_240!();