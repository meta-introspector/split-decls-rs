macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < A : Eq , const N : usize > Eq for RingBuffer < A , N > { }
    };
}

impl_223!()