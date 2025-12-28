macro_rules! deps {
    () => {
        TinyVecIterator!();
        Array!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < A : Array > FusedIterator for TinyVecIterator < A > { }
    };
}

impl_156!();