// Generated macro for How (enum)
macro_rules! Depcrate_runtimeHow {
() => {
// Module: crate::runtime
// Provides: {"How"}
// Dependencies: {}
# [doc = " `SIG_*` constants for use with [`kernel_sigprocmask`]."] # [repr (u32)] pub enum How { # [doc = " `SIG_BLOCK`"] BLOCK = linux_raw_sys :: general :: SIG_BLOCK , # [doc = " `SIG_UNBLOCK`"] UNBLOCK = linux_raw_sys :: general :: SIG_UNBLOCK , # [doc = " `SIG_SETMASK`"] SETMASK = linux_raw_sys :: general :: SIG_SETMASK , }
};
}
