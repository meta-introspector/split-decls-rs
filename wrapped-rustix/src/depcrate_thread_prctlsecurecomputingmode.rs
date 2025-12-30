// Generated macro for SecureComputingMode (enum)
macro_rules! Depcrate_thread_prctlSecureComputingMode {
() => {
// Module: crate::thread::prctl
// Provides: {"SecureComputingMode"}
// Dependencies: {}
# [doc = " `SECCOMP_MODE_*`"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (i32)] pub enum SecureComputingMode { # [doc = " Secure computing is not in use."] Disabled = SECCOMP_MODE_DISABLED , # [doc = " Use hard-coded filter."] Strict = SECCOMP_MODE_STRICT , # [doc = " Use user-supplied filter."] Filter = SECCOMP_MODE_FILTER , }
};
}
