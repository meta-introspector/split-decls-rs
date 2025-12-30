// Generated macro for impl_1867 (impl)
macro_rules! Depcrate_thread_prctlimpl_1867 {
() => {
// Module: crate::thread::prctl
// Provides: {"impl_1867"}
// Dependencies: {}
impl TryFrom < i32 > for SecureComputingMode { type Error = io :: Errno ; fn try_from (value : i32) -> Result < Self , Self :: Error > { match value { SECCOMP_MODE_DISABLED => Ok (Self :: Disabled) , SECCOMP_MODE_STRICT => Ok (Self :: Strict) , SECCOMP_MODE_FILTER => Ok (Self :: Filter) , _ => Err (io :: Errno :: RANGE) , } } }
};
}
