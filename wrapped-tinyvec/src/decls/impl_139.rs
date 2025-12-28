macro_rules! deps {
    () => {
        Array!();
        TinyVecSplice!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < 'p , A , I > ExactSizeIterator for TinyVecSplice < 'p , A , I > where A : Array , I : Iterator < Item = A :: Item > , { # [inline] fn len (& self) -> usize { self . removal_end - self . removal_start } }
    };
}

impl_139!();