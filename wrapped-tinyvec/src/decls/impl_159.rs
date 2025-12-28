macro_rules! deps {
    () => {
        Array!();
        TinyVecIterator!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < A : Array > ExactSizeIterator for TinyVecIterator < A > { impl_mirrored ! { type Mirror = TinyVecIterator ; # [inline] fn len (self : & Self) -> usize ; } }
    };
}

impl_159!()