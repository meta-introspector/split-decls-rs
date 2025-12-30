// Generated macro for impl_147 (impl)
macro_rules! Depcrate_sparse_chunk_refpoolimpl_147 {
() => {
// Module: crate::sparse_chunk::refpool
// Provides: {"impl_147"}
// Dependencies: {}
impl < A , const N : usize > PoolDefault for SparseChunk < A , N > where BitsImpl < N > : Bits , { unsafe fn default_uninit (target : & mut MaybeUninit < Self >) { let ptr = target . as_mut_ptr () ; let map_ptr : * mut Bitmap < N > = & mut (* ptr) . map ; map_ptr . write (Bitmap :: new ()) ; } }
};
}
