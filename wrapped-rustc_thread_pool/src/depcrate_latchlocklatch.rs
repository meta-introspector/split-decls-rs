// Generated macro for LockLatch (struct)
macro_rules! Depcrate_latchLockLatch {
() => {
// Module: crate::latch
// Provides: {"LockLatch"}
// Dependencies: {}
# [doc = " A Latch starts as false and eventually becomes true. You can block"] # [doc = " until it becomes true."] # [derive (Debug)] pub (super) struct LockLatch { m : Mutex < bool > , v : Condvar , }
};
}
