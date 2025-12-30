// Generated macro for AtomicOwned (struct)
macro_rules! Depcrate_atomic_ownedAtomicOwned {
() => {
// Module: crate::atomic_owned
// Provides: {"AtomicOwned"}
// Dependencies: {}
# [doc = " [`AtomicOwned`] owns the underlying instance, and allows users to perform atomic operations"] # [doc = " on the pointer to it."] # [derive (Debug)] pub struct AtomicOwned < T > { instance_ptr : AtomicPtr < RefCounted < T > > , }
};
}
