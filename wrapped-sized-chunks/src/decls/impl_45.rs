macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > ExactSizeIterator for Drain < 'a , A , N > where A : 'a { }
    };
}

impl_45!()