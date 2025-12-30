// Generated macro for WaitTimeoutResult (struct)
macro_rules! Depcrate_syncWaitTimeoutResult {
() => {
// Module: crate::sync
// Provides: {"WaitTimeoutResult"}
// Dependencies: {}
# [doc = " A type indicating whether a timed wait on a condition variable returned"] # [doc = " due to a time out or not."] # [doc = ""] # [doc = " It is returned by the [`wait_timeout`] method."] # [doc = ""] # [doc = " [`wait_timeout`]: Condvar::wait_timeout"] # [derive (Debug , PartialEq , Eq , Copy , Clone)] # [stable (feature = "wait_timeout" , since = "1.5.0")] pub struct WaitTimeoutResult (bool) ;
};
}
