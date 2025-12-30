// Generated macro for impl_156 (impl)
macro_rules! Depcrate_offset_arcimpl_156 {
() => {
// Module: crate::offset_arc
// Provides: {"impl_156"}
// Dependencies: {}
impl < T > Deref for OffsetArc < T > { type Target = T ; # [inline] fn deref (& self) -> & Self :: Target { unsafe { & * self . ptr . as_ptr () } } }
};
}
