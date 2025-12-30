// Generated macro for impl_324 (impl)
macro_rules! Depcrate_ring_bufferimpl_324 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_324"}
// Dependencies: {}
impl < 'a , A , const N : usize > IntoIterator for & 'a RingBuffer < A , N > { type Item = & 'a A ; type IntoIter = Iter < 'a , A , N > ; # [inline] # [must_use] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
