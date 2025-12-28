macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < A , T > ExactSizeIterator for Iter < A , T > { }
    };
}

impl_3!()