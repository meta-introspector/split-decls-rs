// Generated macro for set_secure_computing_mode (function)
macro_rules! Depcrate_thread_prctlset_secure_computing_mode {
() => {
// Module: crate::thread::prctl
// Provides: {"set_secure_computing_mode"}
// Dependencies: {}
# [doc = " Set the secure computing mode for the calling thread, to limit the"] # [doc = " available system calls."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_SECCOMP,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_SECCOMP,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn set_secure_computing_mode (mode : SecureComputingMode) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_SECCOMP , mode as usize as * mut _) } . map (| _r | ()) }
};
}
