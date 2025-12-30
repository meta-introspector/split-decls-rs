// Generated macro for impl_259 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_259 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_259"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > Index < usize > for Slice < 'a , A , N > { type Output = A ; # [inline] # [must_use] fn index (& self , index : usize) -> & Self :: Output { self . buffer . index (self . range . start + index) } }
};
}
