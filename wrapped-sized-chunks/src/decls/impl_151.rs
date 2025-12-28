macro_rules! deps {
    () => {
        IterMut!();
        RawIndex!();
        RingBuffer!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > IterMut < 'a , A , N > where A : 'a , { pub (crate) fn new (buffer : & mut RingBuffer < A , N >) -> Self { Self :: new_slice (buffer , buffer . origin , buffer . len ()) } pub (crate) fn new_slice (buffer : & mut RingBuffer < A , N > , origin : RawIndex < N > , len : usize ,) -> Self { Self { left_index : origin , right_index : origin + len , remaining : len , phantom : PhantomData , data : buffer . data . as_mut_ptr () . cast () , } } unsafe fn mut_ptr (& mut self , index : RawIndex < N >) -> * mut A { self . data . add (index . to_usize ()) } }
    };
}

impl_151!()