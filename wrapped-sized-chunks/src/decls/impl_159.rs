macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > ExactSizeIterator for Drain < 'a , A , N > { }
    };
}

impl_159!();