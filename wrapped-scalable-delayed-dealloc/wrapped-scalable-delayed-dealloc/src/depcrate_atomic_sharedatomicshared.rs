// Generated macro for AtomicShared (struct)
macro_rules! Depcrate_atomic_sharedAtomicShared {
() => {
// Module: crate::atomic_shared
// Provides: {"AtomicShared"}
// Dependencies: {}
# [doc = " [`AtomicShared`] owns the underlying instance, and allows users to perform atomic operations"] # [doc = " on the pointer to it."] # [derive (Debug)] pub struct AtomicShared < T > { instance_ptr : AtomicPtr < RefCounted < T > > , }
};
}
