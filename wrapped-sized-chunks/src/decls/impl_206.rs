macro_rules! deps {
    () => {
        RingBuffer!();
        RawIndex!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < A , const N : usize > PoolClone for RingBuffer < A , N > where A : Clone , { unsafe fn clone_uninit (& self , target : & mut MaybeUninit < Self >) { let ptr = target . as_mut_ptr () ; let origin_ptr : * mut RawIndex < N > = & mut (* ptr) . origin ; let length_ptr : * mut usize = & mut (* ptr) . length ; let data_ptr : * mut _ = & mut (* ptr) . data ; let data_ptr : * mut A = (* data_ptr) . as_mut_ptr () . cast () ; origin_ptr . write (self . origin) ; length_ptr . write (self . length) ; for index in self . range () { data_ptr . add (index . to_usize ()) . write ((* self . ptr (index)) . clone ()) ; } } }
    };
}

impl_206!();