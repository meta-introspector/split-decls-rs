// Generated macro for timing_method (function)
macro_rules! Depcrate_process_prctltiming_method {
() => {
// Module: crate::process::prctl
// Provides: {"timing_method"}
// Dependencies: {}
# [doc = " Get which process timing method is currently in use."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_TIMING,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_TIMING,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_TIMING")] pub fn timing_method () -> io :: Result < TimingMethod > { unsafe { prctl_1arg (PR_GET_TIMING) } . and_then (TryInto :: try_into) }
};
}
