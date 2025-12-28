macro_rules! deps {
    () => {
        SparseChunk!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < A , const N : usize > PoolDefault for SparseChunk < A , N > where BitsImpl < N > : Bits , { unsafe fn default_uninit (target : & mut MaybeUninit < Self >) { let ptr = target . as_mut_ptr () ; let map_ptr : * mut Bitmap < N > = & mut (* ptr) . map ; map_ptr . write (Bitmap :: new ()) ; } }
    };
}

impl_97!()