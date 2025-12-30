// Generated macro for impl_276 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_276 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_276"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > Into < Slice < 'a , A , N > > for SliceMut < 'a , A , N > { # [inline] # [must_use] fn into (self) -> Slice < 'a , A , N > { self . unmut () } }
};
}
