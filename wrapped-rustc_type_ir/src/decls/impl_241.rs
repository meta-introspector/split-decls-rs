macro_rules! deps {
    () => {
        GenericArg!();
        IterInstantiated!();
        Interner!();
        TypeFoldable!();
        SliceLike!();
        EarlyBinder!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < I : Interner , Iter : IntoIterator , A > DoubleEndedIterator for IterInstantiated < I , Iter , A > where Iter :: IntoIter : DoubleEndedIterator , Iter :: Item : TypeFoldable < I > , A : SliceLike < Item = I :: GenericArg > , { fn next_back (& mut self) -> Option < Self :: Item > { Some (EarlyBinder { value : self . it . next_back () ? , _tcx : PhantomData } . instantiate (self . cx , self . args) ,) } }
    };
}

impl_241!();