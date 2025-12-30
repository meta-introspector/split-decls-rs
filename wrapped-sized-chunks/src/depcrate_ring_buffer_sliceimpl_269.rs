// Generated macro for impl_269 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_269 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_269"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > IntoIterator for & 'a Slice < 'a , A , N > { type Item = & 'a A ; type IntoIter = Iter < 'a , A , N > ; # [inline] # [must_use] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
