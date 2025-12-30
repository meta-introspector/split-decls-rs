// Generated macro for impl_98 (impl)
macro_rules! Depcrate_slicevecimpl_98 {
() => {
// Module: crate::slicevec
// Provides: {"impl_98"}
// Dependencies: {}
impl < 's , T > Deref for SliceVec < 's , T > { type Target = [T] ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . data [.. self . len] } }
};
}
