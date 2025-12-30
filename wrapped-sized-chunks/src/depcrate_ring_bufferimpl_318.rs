// Generated macro for impl_318 (impl)
macro_rules! Depcrate_ring_bufferimpl_318 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_318"}
// Dependencies: {}
impl < A : Debug , const N : usize > Debug for RingBuffer < A , N > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . write_str ("RingBuffer") ? ; f . debug_list () . entries (self . iter ()) . finish () } }
};
}
