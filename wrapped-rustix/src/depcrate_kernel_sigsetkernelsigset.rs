// Generated macro for KernelSigSet (struct)
macro_rules! Depcrate_kernel_sigsetKernelSigSet {
() => {
// Module: crate::kernel_sigset
// Provides: {"KernelSigSet"}
// Dependencies: {}
# [doc = " `kernel_sigset_t`—A set of signal numbers, as used by some syscalls."] # [doc = ""] # [doc = " This is similar to `libc::sigset_t`, but with only enough space for the"] # [doc = " signals currently known to be used by the kernel. libc implementations"] # [doc = " reserve extra space so that if Linux defines new signals in the future"] # [doc = " they can add support without breaking their dynamic linking ABI. Rustix"] # [doc = " doesn't support a dynamic linking ABI, so if we need to increase the"] # [doc = " size of `KernelSigSet` in the future, we can do so."] # [doc = ""] # [doc = " It's also the case that the last time Linux changed the size of its"] # [doc = " `kernel_sigset_t` was when it added support for POSIX.1b signals in 1999."] # [doc = ""] # [doc = " `KernelSigSet` is guaranteed to have a subset of the layout of"] # [doc = " `libc::sigset_t`."] # [doc = ""] # [doc = " libc implementations typically also reserve some signal values for internal"] # [doc = " use. In a process that contains a libc, some unsafe functions invoke"] # [doc = " undefined behavior if passed a `KernelSigSet` that contains one of the"] # [doc = " signals that the libc reserves."] # [repr (transparent)] # [derive (Clone)] pub struct KernelSigSet (kernel_sigset_t) ;
};
}
