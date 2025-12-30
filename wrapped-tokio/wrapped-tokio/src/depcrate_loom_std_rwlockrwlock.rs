// Generated macro for RwLock (struct)
macro_rules! Depcrate_loom_std_rwlockRwLock {
() => {
// Module: crate::loom::std::rwlock
// Provides: {"RwLock"}
// Dependencies: {}
# [doc = " Adapter for `std::sync::RwLock` that removes the poisoning aspects"] # [doc = " from its api."] # [derive (Debug)] pub (crate) struct RwLock < T : ? Sized > (sync :: RwLock < T >) ;
};
}
