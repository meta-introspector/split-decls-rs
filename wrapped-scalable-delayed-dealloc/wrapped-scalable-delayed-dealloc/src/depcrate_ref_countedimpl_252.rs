// Generated macro for impl_252 (impl)
macro_rules! Depcrate_ref_countedimpl_252 {
() => {
// Module: crate::ref_counted
// Provides: {"impl_252"}
// Dependencies: {}
impl < T > Deref for RefCounted < T > { type Target = T ; # [inline] fn deref (& self) -> & Self :: Target { & self . instance } }
};
}
