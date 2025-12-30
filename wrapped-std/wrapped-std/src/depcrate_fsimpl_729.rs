// Generated macro for impl_729 (impl)
macro_rules! Depcrate_fsimpl_729 {
() => {
// Module: crate::fs
// Provides: {"impl_729"}
// Dependencies: {}
# [stable (feature = "file_lock" , since = "1.89.0")] impl From < TryLockError > for io :: Error { fn from (err : TryLockError) -> io :: Error { match err { TryLockError :: Error (err) => err , TryLockError :: WouldBlock => io :: ErrorKind :: WouldBlock . into () , } } }
};
}
