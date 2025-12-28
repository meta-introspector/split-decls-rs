macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < A , T > FusedIterator for Iter < A , T > { }
    };
}

impl_4!()