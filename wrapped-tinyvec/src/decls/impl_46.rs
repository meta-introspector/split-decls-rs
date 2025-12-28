macro_rules! deps {
    () => {
        Array!();
        ArrayVecIterator!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < A : Array > ExactSizeIterator for ArrayVecIterator < A > { # [inline] fn len (& self) -> usize { self . size_hint () . 0 } }
    };
}

impl_46!();