macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < A , const N : usize > PoolDefault for Chunk < A , N > { unsafe fn default_uninit (target : & mut MaybeUninit < Self >) { let ptr = target . as_mut_ptr () ; let left_ptr : * mut usize = & mut (* ptr) . left ; let right_ptr : * mut usize = & mut (* ptr) . right ; left_ptr . write (0) ; right_ptr . write (0) ; } }
    };
}

impl_48!();