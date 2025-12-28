macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > FusedIterator for Drain < 'a , A , N > { }
    };
}

impl_160!()