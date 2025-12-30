// Generated macro for impl_311 (impl)
macro_rules! Depcrate_ring_bufferimpl_311 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_311"}
// Dependencies: {}
impl < A , const N : usize > PartialEq < Slice < '_ , A , N > > for RingBuffer < A , N > where A : PartialEq , { fn eq (& self , other : & Slice < '_ , A , N >) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
