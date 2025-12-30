// Generated macro for impl_325 (impl)
macro_rules! Depcrate_ring_bufferimpl_325 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_325"}
// Dependencies: {}
impl < 'a , A , const N : usize > IntoIterator for & 'a mut RingBuffer < A , N > { type Item = & 'a mut A ; type IntoIter = IterMut < 'a , A , N > ; # [inline] # [must_use] fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
