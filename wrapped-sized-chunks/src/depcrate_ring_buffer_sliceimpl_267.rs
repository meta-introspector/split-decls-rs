// Generated macro for impl_267 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_267 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_267"}
// Dependencies: {}
impl < 'a , A : Debug + 'a , const N : usize > Debug for Slice < 'a , A , N > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . write_str ("RingBuffer") ? ; f . debug_list () . entries (self . iter ()) . finish () } }
};
}
