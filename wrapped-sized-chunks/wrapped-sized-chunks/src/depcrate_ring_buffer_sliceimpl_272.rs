// Generated macro for impl_272 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_272 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > Array for SliceMut < 'a , A , N > { # [doc = " Get a reference to the value at a given index."] # [inline] # [must_use] fn get (& self , index : usize) -> Option < & A > { if index >= self . len () { None } else { Some (unsafe { self . get_unchecked (index) }) } } }
};
}
