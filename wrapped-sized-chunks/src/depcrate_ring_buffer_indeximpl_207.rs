// Generated macro for impl_207 (impl)
macro_rules! Depcrate_ring_buffer_indeximpl_207 {
() => {
// Module: crate::ring_buffer::index
// Provides: {"impl_207"}
// Dependencies: {}
impl < const N : usize > SubAssign < usize > for RawIndex < N > { # [inline] fn sub_assign (& mut self , other : usize) { while other > self . 0 { self . 0 += N ; } self . 0 -= other ; } }
};
}
