macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > FusedIterator for Drain < 'a , A , N > where A : 'a { }
    };
}

impl_46!();