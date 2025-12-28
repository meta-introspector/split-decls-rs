macro_rules! deps {
    () => {
        Array!();
        ArrayVecIterator!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < A : Array > FusedIterator for ArrayVecIterator < A > { }
    };
}

impl_43!();