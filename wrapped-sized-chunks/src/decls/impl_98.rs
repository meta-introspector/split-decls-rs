macro_rules! deps {
    () => {
        SparseChunk!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < A , const N : usize > PoolClone for SparseChunk < A , N > where A : Clone , BitsImpl < N > : Bits , { unsafe fn clone_uninit (& self , target : & mut MaybeUninit < Self >) { let ptr = target . as_mut_ptr () ; let map_ptr : * mut Bitmap < N > = & mut (* ptr) . map ; let data_ptr : * mut _ = & mut (* ptr) . data ; let data_ptr : * mut A = (* data_ptr) . as_mut_ptr () . cast () ; map_ptr . write (self . map) ; for index in & self . map { data_ptr . add (index) . write (self [index] . clone ()) ; } } }
    };
}

impl_98!();