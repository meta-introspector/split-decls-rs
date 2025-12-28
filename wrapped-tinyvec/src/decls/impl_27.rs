macro_rules! deps {
    () => {
        Array!();
        ArrayVecSplice!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < 'p , A , I > FusedIterator for ArrayVecSplice < 'p , A , I > where A : Array , I : Iterator < Item = A :: Item > , { }
    };
}

impl_27!()