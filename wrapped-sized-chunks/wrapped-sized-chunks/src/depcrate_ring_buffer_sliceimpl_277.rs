// Generated macro for impl_277 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_277 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_277"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > Index < usize > for SliceMut < 'a , A , N > { type Output = A ; # [inline] # [must_use] fn index (& self , index : usize) -> & Self :: Output { self . buffer . index (self . range . start + index) } }
};
}
