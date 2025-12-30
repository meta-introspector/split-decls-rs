// Generated macro for impl_206 (impl)
macro_rules! Depcrate_ring_buffer_indeximpl_206 {
() => {
// Module: crate::ring_buffer::index
// Provides: {"impl_206"}
// Dependencies: {}
impl < const N : usize > Sub < usize > for RawIndex < N > { type Output = RawIndex < N > ; # [inline] # [must_use] fn sub (self , other : usize) -> Self :: Output { let mut start = self . 0 ; while other > start { start += N ; } (start - other) . into () } }
};
}
