// Generated macro for impl_727 (impl)
macro_rules! Depcrate_fsimpl_727 {
() => {
// Module: crate::fs
// Provides: {"impl_727"}
// Dependencies: {}
# [stable (feature = "file_lock" , since = "1.89.0")] impl fmt :: Debug for TryLockError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { TryLockError :: Error (err) => err . fmt (f) , TryLockError :: WouldBlock => "WouldBlock" . fmt (f) , } } }
};
}
