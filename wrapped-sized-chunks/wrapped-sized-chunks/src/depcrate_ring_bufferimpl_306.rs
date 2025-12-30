// Generated macro for impl_306 (impl)
macro_rules! Depcrate_ring_bufferimpl_306 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_306"}
// Dependencies: {}
impl < A : Clone , const N : usize > Clone for RingBuffer < A , N > { fn clone (& self) -> Self { let mut out = Self :: new () ; out . origin = self . origin ; out . length = self . length ; let range = self . range () ; out . length = 0 ; for index in range { unsafe { out . force_write (index , (& * self . ptr (index)) . clone ()) } ; out . length += 1 ; } out } }
};
}
