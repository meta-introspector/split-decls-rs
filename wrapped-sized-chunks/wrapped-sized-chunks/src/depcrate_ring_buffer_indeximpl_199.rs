// Generated macro for impl_199 (impl)
macro_rules! Depcrate_ring_buffer_indeximpl_199 {
() => {
// Module: crate::ring_buffer::index
// Provides: {"impl_199"}
// Dependencies: {}
impl < const N : usize > From < usize > for RawIndex < N > { # [inline] # [must_use] fn from (index : usize) -> Self { debug_assert ! (index < N) ; RawIndex (index) } }
};
}
