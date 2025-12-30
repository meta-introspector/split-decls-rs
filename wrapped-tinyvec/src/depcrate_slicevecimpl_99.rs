// Generated macro for impl_99 (impl)
macro_rules! Depcrate_slicevecimpl_99 {
() => {
// Module: crate::slicevec
// Provides: {"impl_99"}
// Dependencies: {}
impl < 's , T > DerefMut for SliceVec < 's , T > { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . data [.. self . len] } }
};
}
