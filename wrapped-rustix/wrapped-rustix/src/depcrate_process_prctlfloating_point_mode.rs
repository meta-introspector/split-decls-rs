// Generated macro for floating_point_mode (function)
macro_rules! Depcrate_process_prctlfloating_point_mode {
() => {
// Module: crate::process::prctl
// Provides: {"floating_point_mode"}
// Dependencies: {}
# [doc = " Get the current floating point mode."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_FP_MODE,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_FP_MODE,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_FP_MODE")] pub fn floating_point_mode () -> io :: Result < FloatingPointMode > { let r = unsafe { prctl_1arg (PR_GET_FP_MODE) ? } as c_uint ; FloatingPointMode :: try_from (r) }
};
}
