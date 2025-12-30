// Generated macro for impl_278 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_278 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_278"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > IndexMut < usize > for SliceMut < 'a , A , N > { # [inline] # [must_use] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { self . buffer . index_mut (self . range . start + index) } }
};
}
