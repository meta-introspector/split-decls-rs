// Generated macro for impl_204 (impl)
macro_rules! Depcrate_ring_buffer_indeximpl_204 {
() => {
// Module: crate::ring_buffer::index
// Provides: {"impl_204"}
// Dependencies: {}
impl < const N : usize > AddAssign < usize > for RawIndex < N > { # [inline] fn add_assign (& mut self , other : usize) { self . 0 += other ; while self . 0 >= N { self . 0 -= N ; } } }
};
}
