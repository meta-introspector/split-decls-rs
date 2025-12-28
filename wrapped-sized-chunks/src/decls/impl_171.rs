macro_rules! deps {
    () => {
        RingBuffer!();
        Slice!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > From < & 'a RingBuffer < A , N > > for Slice < 'a , A , N > { # [inline] # [must_use] fn from (buffer : & 'a RingBuffer < A , N >) -> Self { Slice { range : Range { start : 0 , end : buffer . len () , } , buffer , } } }
    };
}

impl_171!();