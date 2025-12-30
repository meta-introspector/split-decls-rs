// Generated macro for child_subreaper (function)
macro_rules! Depcrate_process_prctlchild_subreaper {
() => {
// Module: crate::process::prctl
// Provides: {"child_subreaper"}
// Dependencies: {}
# [doc = " Get the `child subreaper` setting of the calling process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_CHILD_SUBREAPER,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_CHILD_SUBREAPER,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_CHILD_SUBREAPER")] pub fn child_subreaper () -> io :: Result < Option < Pid > > { unsafe { let r = prctl_get_at_arg2_optional :: < c_uint > (PR_GET_CHILD_SUBREAPER) ? ; Ok (Pid :: from_raw (r as RawPid)) } }
};
}
