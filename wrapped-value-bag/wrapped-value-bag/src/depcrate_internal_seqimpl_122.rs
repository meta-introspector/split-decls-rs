// Generated macro for impl_122 (impl)
macro_rules! Depcrate_internal_seqimpl_122 {
() => {
// Module: crate::internal::seq
// Provides: {"impl_122"}
// Dependencies: {}
impl < T : Seq + 'static > DowncastSeq for T { fn as_any (& self) -> & dyn Any { self } fn as_super (& self) -> & dyn Seq { self } }
};
}
