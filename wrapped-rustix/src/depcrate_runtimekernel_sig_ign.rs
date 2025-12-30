// Generated macro for kernel_sig_ign (function)
macro_rules! Depcrate_runtimekernel_sig_ign {
() => {
// Module: crate::runtime
// Provides: {"kernel_sig_ign"}
// Dependencies: {}
# [doc = " Return a special “ignore” signal handler for ignoring signals."] # [doc = ""] # [doc = " This isn't the `SIG_IGN` value itself; it's a function that returns the"] # [doc = " `SIG_IGN` value."] # [doc = ""] # [doc = " If you're looking for `kernel_sig_dfl`; use [`KERNEL_SIG_DFL`]."] # [doc (alias = "SIG_IGN")] # [must_use] pub const fn kernel_sig_ign () -> KernelSighandler { linux_raw_sys :: signal_macros :: sig_ign () }
};
}
