// Generated macro for impl_203 (impl)
macro_rules! Depcrate_ring_buffer_indeximpl_203 {
() => {
// Module: crate::ring_buffer::index
// Provides: {"impl_203"}
// Dependencies: {}
impl < const N : usize > Add < usize > for RawIndex < N > { type Output = RawIndex < N > ; # [inline] # [must_use] fn add (self , other : usize) -> Self :: Output { let mut result = self . 0 + other ; while result >= N { result -= N ; } result . into () } }
};
}
