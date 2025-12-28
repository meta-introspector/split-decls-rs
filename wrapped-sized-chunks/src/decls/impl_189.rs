macro_rules! deps {
    () => {
        SliceMut!();
        RingBuffer!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > From < & 'a mut RingBuffer < A , N > > for SliceMut < 'a , A , N > { # [must_use] fn from (buffer : & 'a mut RingBuffer < A , N >) -> Self { SliceMut { range : Range { start : 0 , end : buffer . len () , } , buffer , } } }
    };
}

impl_189!();