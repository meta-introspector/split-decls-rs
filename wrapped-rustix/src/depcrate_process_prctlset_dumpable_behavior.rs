// Generated macro for set_dumpable_behavior (function)
macro_rules! Depcrate_process_prctlset_dumpable_behavior {
() => {
// Module: crate::process::prctl
// Provides: {"set_dumpable_behavior"}
// Dependencies: {}
# [doc = " Set the state of the `dumpable` attribute."] # [doc = ""] # [doc = " This attribute determines whether the process can be traced and whether"] # [doc = " core dumps are produced for the calling process upon delivery of a signal"] # [doc = " whose default behavior is to produce a core dump."] # [doc = ""] # [doc = " A similar function with the same name is available on FreeBSD (as part of"] # [doc = " the `procctl` interface), but it has an extra argument which allows to"] # [doc = " select a process other then the current process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_DUMPABLE,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_DUMPABLE,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_DUMPABLE")] pub fn set_dumpable_behavior (config : DumpableBehavior) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_DUMPABLE , config as usize as * mut _) } . map (| _r | ()) }
};
}
