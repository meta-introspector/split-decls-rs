// Generated macro for impl_125 (impl)
macro_rules! Depcrate_latchimpl_125 {
() => {
// Module: crate::latch
// Provides: {"impl_125"}
// Dependencies: {}
impl Latch for LockLatch { # [inline] unsafe fn set (this : * const Self) { let mut guard = unsafe { (* this) . m . lock () . unwrap () } ; * guard = true ; unsafe { (* this) . v . notify_all () } ; } }
};
}
