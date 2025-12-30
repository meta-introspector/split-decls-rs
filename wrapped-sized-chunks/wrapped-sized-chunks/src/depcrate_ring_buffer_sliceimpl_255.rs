// Generated macro for impl_255 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_255 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_255"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > Array for Slice < 'a , A , N > { # [doc = " Get a reference to the value at a given index."] # [inline] # [must_use] fn get (& self , index : usize) -> Option < & A > { if index >= self . len () { None } else { Some (unsafe { self . get_unchecked (index) }) } } }
};
}
