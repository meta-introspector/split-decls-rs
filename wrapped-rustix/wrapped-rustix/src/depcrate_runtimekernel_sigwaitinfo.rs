// Generated macro for kernel_sigwaitinfo (function)
macro_rules! Depcrate_runtimekernel_sigwaitinfo {
() => {
// Module: crate::runtime
// Provides: {"kernel_sigwaitinfo"}
// Dependencies: {}
# [doc = " `sigwaitinfo(set)`—Wait for signals, returning a [`Siginfo`]."] # [doc = ""] # [doc = " If this is ever exposed publicly, we should think about whether it should"] # [doc = " mask out signals reserved by libc."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If there is a libc in the process, the `set` must not contain any signal"] # [doc = " reserved by the libc."] # [doc = ""] # [doc = " If code elsewhere in the program is depending on delivery of a signal for"] # [doc = " any reason, for example to prevent it from executing some code, this could"] # [doc = " cause it to miss that signal, and for example execute that code. And if the"] # [doc = " caller is depending on delivery of a signal for any reason, its assumptions"] # [doc = " could be broken by code elsewhere in the program calling this function."] # [doc = ""] # [doc = " There may be further safety hazards not yet documented here."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/sigwaitinfo.2.html"] # [inline] pub unsafe fn kernel_sigwaitinfo (set : & KernelSigSet) -> io :: Result < Siginfo > { backend :: runtime :: syscalls :: kernel_sigwaitinfo (set) }
};
}
