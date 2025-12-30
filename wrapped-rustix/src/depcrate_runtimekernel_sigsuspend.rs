// Generated macro for kernel_sigsuspend (function)
macro_rules! Depcrate_runtimekernel_sigsuspend {
() => {
// Module: crate::runtime
// Provides: {"kernel_sigsuspend"}
// Dependencies: {}
# [doc = " `sigsuspend(set)`—Suspend the calling thread and wait for signals."] # [doc = ""] # [doc = " If this is ever exposed publicly, we should think about whether it should"] # [doc = " be made to fail if given signals reserved by libc."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `sigsuspend`]"] # [doc = ""] # [doc = " [Linux `sigsuspend`]: https://man7.org/linux/man-pages/man2/sigsuspend.2.html"] # [inline] pub fn kernel_sigsuspend (set : & KernelSigSet) -> io :: Result < () > { backend :: runtime :: syscalls :: kernel_sigsuspend (set) }
};
}
