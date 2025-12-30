// Generated macro for impl_286 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_286 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_286"}
// Dependencies: {}
impl < 'a , A : Debug + 'a , const N : usize > Debug for SliceMut < 'a , A , N > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . write_str ("RingBuffer") ? ; f . debug_list () . entries (self . iter ()) . finish () } }
};
}
