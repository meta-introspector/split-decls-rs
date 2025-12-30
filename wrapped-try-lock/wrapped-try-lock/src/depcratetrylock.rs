// Generated macro for TryLock (struct)
macro_rules! DepcrateTryLock {
() => {
// Module: crate
// Provides: {"TryLock"}
// Dependencies: {}
# [doc = " A light-weight lock guarded by an atomic boolean."] # [doc = ""] # [doc = " Most efficient when contention is low, acquiring the lock is a single"] # [doc = " atomic swap, and releasing it just 1 more atomic swap."] # [doc = ""] # [doc = " It is only possible to try to acquire the lock, it is not possible to"] # [doc = " wait for the lock to become ready, like with a `Mutex`."] # [derive (Default)] pub struct TryLock < T > { is_locked : AtomicBool , value : UnsafeCell < T > , }
};
}
