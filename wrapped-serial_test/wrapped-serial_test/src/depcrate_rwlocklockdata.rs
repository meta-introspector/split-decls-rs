// Generated macro for LockData (struct)
macro_rules! Depcrate_rwlockLockData {
() => {
// Module: crate::rwlock
// Provides: {"LockData"}
// Dependencies: {}
struct LockData { mutex : Mutex < LockState > , serial : ReentrantMutex < () > , condvar : Condvar , }
};
}
