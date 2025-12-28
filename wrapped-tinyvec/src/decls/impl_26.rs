macro_rules! deps {
    () => {
        Array!();
        ArrayVecSplice!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'p , A , I > ExactSizeIterator for ArrayVecSplice < 'p , A , I > where A : Array , I : Iterator < Item = A :: Item > , { # [inline] fn len (& self) -> usize { self . removal_end - self . removal_start } }
    };
}

impl_26!()