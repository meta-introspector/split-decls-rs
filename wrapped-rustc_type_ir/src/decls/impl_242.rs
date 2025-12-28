macro_rules! deps {
    () => {
        IterInstantiated!();
        TypeFoldable!();
        SliceLike!();
        GenericArg!();
        Interner!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl < I : Interner , Iter : IntoIterator , A > ExactSizeIterator for IterInstantiated < I , Iter , A > where Iter :: IntoIter : ExactSizeIterator , Iter :: Item : TypeFoldable < I > , A : SliceLike < Item = I :: GenericArg > , { }
    };
}

impl_242!();