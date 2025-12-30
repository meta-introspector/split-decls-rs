// Generated macro for impl_302 (impl)
macro_rules! Depcrate_ring_bufferimpl_302 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_302"}
// Dependencies: {}
impl < A , const N : usize > Array for RingBuffer < A , N > { # [doc = " Get a reference to the value at a given index."] # [must_use] fn get (& self , index : usize) -> Option < & A > { if index >= self . len () { None } else { Some (unsafe { self . get_unchecked (index) }) } } }
};
}
