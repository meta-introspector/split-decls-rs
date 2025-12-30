// Generated macro for impl_273 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_273 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_273"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > ArrayMut for SliceMut < 'a , A , N > { # [doc = " Get a mutable reference to the value at a given index."] # [inline] # [must_use] fn get_mut (& mut self , index : usize) -> Option < & mut A > { if index >= self . len () { None } else { Some (unsafe { self . get_unchecked_mut (index) }) } } }
};
}
