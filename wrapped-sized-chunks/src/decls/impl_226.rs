macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl < A , const N : usize > Extend < A > for RingBuffer < A , N > { # [inline] fn extend < I : IntoIterator < Item = A > > (& mut self , iter : I) { for item in iter { self . push_back (item) ; } } }
    };
}

impl_226!()