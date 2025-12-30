// Generated macro for impl_728 (impl)
macro_rules! Depcrate_fsimpl_728 {
() => {
// Module: crate::fs
// Provides: {"impl_728"}
// Dependencies: {}
# [stable (feature = "file_lock" , since = "1.89.0")] impl fmt :: Display for TryLockError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { TryLockError :: Error (_) => "lock acquisition failed due to I/O error" , TryLockError :: WouldBlock => "lock acquisition failed because the operation would block" , } . fmt (f) } }
};
}
