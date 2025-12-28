macro_rules! deps {
    () => {
        Array!();
        TinyVecSplice!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < 'p , A , I > FusedIterator for TinyVecSplice < 'p , A , I > where A : Array , I : Iterator < Item = A :: Item > , { }
    };
}

impl_140!();