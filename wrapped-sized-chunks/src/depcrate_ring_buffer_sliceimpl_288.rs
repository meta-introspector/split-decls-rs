// Generated macro for impl_288 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_288 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_288"}
// Dependencies: {}
impl < 'a , 'b , A : 'a , const N : usize > IntoIterator for & 'a SliceMut < 'a , A , N > { type Item = & 'a A ; type IntoIter = Iter < 'a , A , N > ; # [inline] # [must_use] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
