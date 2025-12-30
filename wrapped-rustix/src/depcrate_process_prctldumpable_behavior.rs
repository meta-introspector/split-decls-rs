// Generated macro for dumpable_behavior (function)
macro_rules! Depcrate_process_prctldumpable_behavior {
() => {
// Module: crate::process::prctl
// Provides: {"dumpable_behavior"}
// Dependencies: {}
# [doc = " Get the current state of the calling process' `dumpable` attribute."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_DUMPABLE,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_DUMPABLE,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_DUMPABLE")] pub fn dumpable_behavior () -> io :: Result < DumpableBehavior > { unsafe { prctl_1arg (PR_GET_DUMPABLE) } . and_then (TryInto :: try_into) }
};
}
