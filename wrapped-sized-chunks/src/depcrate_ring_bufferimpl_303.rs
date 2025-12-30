// Generated macro for impl_303 (impl)
macro_rules! Depcrate_ring_bufferimpl_303 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_303"}
// Dependencies: {}
impl < A , const N : usize > ArrayMut for RingBuffer < A , N > { # [doc = " Get a mutable reference to the value at a given index."] # [must_use] fn get_mut (& mut self , index : usize) -> Option < & mut A > { if index >= self . len () { None } else { Some (unsafe { self . get_unchecked_mut (index) }) } } }
};
}
