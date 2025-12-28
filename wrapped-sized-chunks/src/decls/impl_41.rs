macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < A , const N : usize > FusedIterator for Iter < A , N > { }
    };
}

impl_41!();