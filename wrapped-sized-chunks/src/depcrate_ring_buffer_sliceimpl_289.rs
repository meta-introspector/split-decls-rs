// Generated macro for impl_289 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_289 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_289"}
// Dependencies: {}
impl < 'a , 'b , A : 'a , const N : usize > IntoIterator for & 'a mut SliceMut < 'a , A , N > { type Item = & 'a mut A ; type IntoIter = IterMut < 'a , A , N > ; # [inline] # [must_use] fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
