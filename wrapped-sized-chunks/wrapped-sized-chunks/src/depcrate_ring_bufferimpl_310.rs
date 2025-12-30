// Generated macro for impl_310 (impl)
macro_rules! Depcrate_ring_bufferimpl_310 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_310"}
// Dependencies: {}
impl < A , PrimSlice , const N : usize > PartialEq < PrimSlice > for RingBuffer < A , N > where PrimSlice : Borrow < [A] > , A : PartialEq , { # [inline] # [must_use] fn eq (& self , other : & PrimSlice) -> bool { let other = other . borrow () ; self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
