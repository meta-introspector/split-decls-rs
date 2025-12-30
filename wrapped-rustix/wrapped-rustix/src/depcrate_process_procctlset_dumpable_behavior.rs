// Generated macro for set_dumpable_behavior (function)
macro_rules! Depcrate_process_procctlset_dumpable_behavior {
() => {
// Module: crate::process::procctl
// Provides: {"set_dumpable_behavior"}
// Dependencies: {}
# [doc = " Set the state of the `dumpable` attribute for the process indicated by"] # [doc = " `idtype` and `id`."] # [doc = ""] # [doc = " This determines whether the process can be traced and whether core dumps"] # [doc = " are produced for the process upon delivery of a signal whose default"] # [doc = " behavior is to produce a core dump."] # [doc = ""] # [doc = " This is similar to `set_dumpable_behavior` on Linux, with the exception"] # [doc = " that on FreeBSD there is an extra argument `process`. When `process` is set"] # [doc = " to `None`, the operation is performed for the current process, like on"] # [doc = " Linux."] # [doc = ""] # [doc = " # References"] # [doc = "  - [FreeBSD `procctl(PROC_TRACE_CTL,…)`]"] # [doc = ""] # [doc = " [FreeBSD `procctl(PROC_TRACE_CTL,…)`]: https://man.freebsd.org/cgi/man.cgi?query=procctl&sektion=2"] # [inline] pub fn set_dumpable_behavior (process : ProcSelector , config : DumpableBehavior) -> io :: Result < () > { unsafe { procctl (PROC_TRACE_CTL , process , config as usize as * mut _) } }
};
}
