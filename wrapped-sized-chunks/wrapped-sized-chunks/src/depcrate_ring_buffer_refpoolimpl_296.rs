// Generated macro for impl_296 (impl)
macro_rules! Depcrate_ring_buffer_refpoolimpl_296 {
() => {
// Module: crate::ring_buffer::refpool
// Provides: {"impl_296"}
// Dependencies: {}
impl < A , const N : usize > PoolDefault for RingBuffer < A , N > { unsafe fn default_uninit (target : & mut MaybeUninit < Self >) { let ptr = target . as_mut_ptr () ; let origin_ptr : * mut RawIndex < N > = & mut (* ptr) . origin ; let length_ptr : * mut usize = & mut (* ptr) . length ; origin_ptr . write (0 . into ()) ; length_ptr . write (0) ; } }
};
}
