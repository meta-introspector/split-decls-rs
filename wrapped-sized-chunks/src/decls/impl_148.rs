macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > ExactSizeIterator for Iter < 'a , A , N > { }
    };
}

impl_148!()