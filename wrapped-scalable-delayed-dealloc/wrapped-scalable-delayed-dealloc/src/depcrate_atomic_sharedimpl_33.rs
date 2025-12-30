// Generated macro for impl_33 (impl)
macro_rules! Depcrate_atomic_sharedimpl_33 {
() => {
// Module: crate::atomic_shared
// Provides: {"impl_33"}
// Dependencies: {}
impl < T > Drop for AtomicShared < T > { # [inline] fn drop (& mut self) { if let Some (ptr) = NonNull :: new (Tag :: unset_tag (self . instance_ptr . load (Relaxed)) . cast_mut ()) { drop (Shared :: from (ptr)) ; } } }
};
}
