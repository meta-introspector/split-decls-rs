// Generated macro for impl_14 (impl)
macro_rules! Depcrate_atomic_ownedimpl_14 {
() => {
// Module: crate::atomic_owned
// Provides: {"impl_14"}
// Dependencies: {}
impl < T > Drop for AtomicOwned < T > { # [inline] fn drop (& mut self) { if let Some (ptr) = NonNull :: new (Tag :: unset_tag (self . instance_ptr . load (Relaxed)) . cast_mut ()) { drop (Owned :: from (ptr)) ; } } }
};
}
