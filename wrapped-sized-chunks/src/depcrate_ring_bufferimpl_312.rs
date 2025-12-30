// Generated macro for impl_312 (impl)
macro_rules! Depcrate_ring_bufferimpl_312 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_312"}
// Dependencies: {}
impl < A , const N : usize > PartialEq < SliceMut < '_ , A , N > > for RingBuffer < A , N > where A : PartialEq , { fn eq (& self , other : & SliceMut < '_ , A , N >) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
