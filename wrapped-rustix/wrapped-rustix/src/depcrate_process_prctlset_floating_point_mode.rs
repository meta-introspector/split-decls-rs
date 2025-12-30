// Generated macro for set_floating_point_mode (function)
macro_rules! Depcrate_process_prctlset_floating_point_mode {
() => {
// Module: crate::process::prctl
// Provides: {"set_floating_point_mode"}
// Dependencies: {}
# [doc = " Allow control of the floating point mode from user space."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_FP_MODE,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_FP_MODE,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_FP_MODE")] pub fn set_floating_point_mode (mode : FloatingPointMode) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_FP_MODE , mode as usize as * mut _) } . map (| _r | ()) }
};
}
