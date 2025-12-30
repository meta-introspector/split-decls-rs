// Generated macro for impl_31 (impl)
macro_rules! Depcrate_atomic_sharedimpl_31 {
() => {
// Module: crate::atomic_shared
// Provides: {"impl_31"}
// Dependencies: {}
impl < T > Clone for AtomicShared < T > { # [inline] fn clone (& self) -> AtomicShared < T > { self . clone (Acquire , & Guard :: new ()) } }
};
}
