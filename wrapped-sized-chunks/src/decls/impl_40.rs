macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < A , const N : usize > ExactSizeIterator for Iter < A , N > { }
    };
}

impl_40!()