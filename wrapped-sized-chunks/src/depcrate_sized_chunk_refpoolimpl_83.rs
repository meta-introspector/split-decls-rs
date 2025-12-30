// Generated macro for impl_83 (impl)
macro_rules! Depcrate_sized_chunk_refpoolimpl_83 {
() => {
// Module: crate::sized_chunk::refpool
// Provides: {"impl_83"}
// Dependencies: {}
impl < A , const N : usize > PoolClone for Chunk < A , N > where A : Clone , { unsafe fn clone_uninit (& self , target : & mut MaybeUninit < Self >) { let ptr = target . as_mut_ptr () ; let left_ptr : * mut usize = & mut (* ptr) . left ; let right_ptr : * mut usize = & mut (* ptr) . right ; let data_ptr : * mut _ = & mut (* ptr) . data ; let data_ptr : * mut A = (* data_ptr) . as_mut_ptr () . cast () ; left_ptr . write (self . left) ; right_ptr . write (self . right) ; for index in self . left .. self . right { data_ptr . add (index) . write ((* self . ptr (index)) . clone ()) ; } } }
};
}
