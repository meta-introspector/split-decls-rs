macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl < A , const N : usize > FromIterator < A > for RingBuffer < A , N > { # [must_use] fn from_iter < I : IntoIterator < Item = A > > (iter : I) -> Self { let mut buffer = RingBuffer :: new () ; buffer . extend (iter) ; buffer } }
    };
}

impl_232!();