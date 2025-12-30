// Generated macro for impl_121 (impl)
macro_rules! Depcrate_ownedimpl_121 {
() => {
// Module: crate::owned
// Provides: {"impl_121"}
// Dependencies: {}
impl < T > Drop for Owned < T > { # [inline] fn drop (& mut self) { RefCounted :: pass_to_collector (self . instance_ptr . as_ptr ()) ; } }
};
}
