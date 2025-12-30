// Generated macro for clear_ambient_capability_set (function)
macro_rules! Depcrate_thread_prctlclear_ambient_capability_set {
() => {
// Module: crate::thread::prctl
// Provides: {"clear_ambient_capability_set"}
// Dependencies: {}
# [doc = " Remove all capabilities from the ambient set."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_CAP_AMBIENT,PR_CAP_AMBIENT_CLEAR_ALL,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_CAP_AMBIENT,PR_CAP_AMBIENT_CLEAR_ALL,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn clear_ambient_capability_set () -> io :: Result < () > { unsafe { prctl_2args (PR_CAP_AMBIENT , PR_CAP_AMBIENT_CLEAR_ALL as * mut _) } . map (| _r | ()) }
};
}
