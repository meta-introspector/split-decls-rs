// Generated macro for impl_158 (impl)
macro_rules! Depcrate_offset_arcimpl_158 {
() => {
// Module: crate::offset_arc
// Provides: {"impl_158"}
// Dependencies: {}
impl < T > Drop for OffsetArc < T > { fn drop (& mut self) { let _ = Arc :: from_raw_offset (OffsetArc { ptr : self . ptr , phantom : PhantomData , }) ; } }
};
}
