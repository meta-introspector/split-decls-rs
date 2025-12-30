// Generated macro for set_child_subreaper (function)
macro_rules! Depcrate_process_prctlset_child_subreaper {
() => {
// Module: crate::process::prctl
// Provides: {"set_child_subreaper"}
// Dependencies: {}
# [doc = " Set the `child subreaper` attribute of the calling process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_CHILD_SUBREAPER,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_CHILD_SUBREAPER,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_CHILD_SUBREAPER")] pub fn set_child_subreaper (pid : Option < Pid >) -> io :: Result < () > { let pid = pid . map_or (0_usize , | pid | pid . as_raw_nonzero () . get () as usize) ; unsafe { prctl_2args (PR_SET_CHILD_SUBREAPER , pid as * mut _) } . map (| _r | ()) }
};
}
