// Generated macro for set_keep_capabilities (function)
macro_rules! Depcrate_thread_prctlset_keep_capabilities {
() => {
// Module: crate::thread::prctl
// Provides: {"set_keep_capabilities"}
// Dependencies: {}
# [doc = " Set the state of the calling thread's `keep capabilities` flag."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_KEEPCAPS,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_KEEPCAPS,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn set_keep_capabilities (enable : bool) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_KEEPCAPS , usize :: from (enable) as * mut _) } . map (| _r | ()) }
};
}
