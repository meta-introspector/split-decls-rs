// Generated macro for impl_323 (impl)
macro_rules! Depcrate_ring_bufferimpl_323 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_323"}
// Dependencies: {}
impl < A , const N : usize > IntoIterator for RingBuffer < A , N > { type Item = A ; type IntoIter = OwnedIter < A , N > ; # [inline] # [must_use] fn into_iter (self) -> Self :: IntoIter { OwnedIter { buffer : self } } }
};
}
