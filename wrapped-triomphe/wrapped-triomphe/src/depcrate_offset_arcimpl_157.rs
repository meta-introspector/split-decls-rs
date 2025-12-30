// Generated macro for impl_157 (impl)
macro_rules! Depcrate_offset_arcimpl_157 {
() => {
// Module: crate::offset_arc
// Provides: {"impl_157"}
// Dependencies: {}
impl < T > Clone for OffsetArc < T > { # [inline] fn clone (& self) -> Self { Arc :: into_raw_offset (self . clone_arc ()) } }
};
}
