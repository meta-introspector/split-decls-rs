// Generated macro for impl_300 (impl)
macro_rules! Depcrate_ring_bufferimpl_300 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_300"}
// Dependencies: {}
impl < A , const N : usize > Drop for RingBuffer < A , N > { # [inline] fn drop (& mut self) { if core :: mem :: needs_drop :: < A > () { for i in self . range () { unsafe { self . force_drop (i) } } } } }
};
}
