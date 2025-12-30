// Generated macro for impl_126 (impl)
macro_rules! Depcrate_internal_seqimpl_126 {
() => {
// Module: crate::internal::seq
// Provides: {"impl_126"}
// Dependencies: {}
impl < 'v , 'a , const N : usize > From < & 'v [& 'a str ; N] > for ValueBag < 'v > { fn from (v : & 'v [& 'a str ; N]) -> Self { ValueBag :: from_seq_slice (v) } }
};
}
