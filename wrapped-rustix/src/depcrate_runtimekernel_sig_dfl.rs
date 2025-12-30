// Generated macro for KERNEL_SIG_DFL (const)
macro_rules! Depcrate_runtimeKERNEL_SIG_DFL {
() => {
// Module: crate::runtime
// Provides: {"KERNEL_SIG_DFL"}
// Dependencies: {}
# [doc = " A special “default” signal handler representing the default behavior"] # [doc = " for handling a signal."] # [doc = ""] # [doc = " If you're looking for `KERNEL_SIG_IGN`; use [`kernel_sig_ign`]."] # [doc (alias = "SIG_DFL")] pub const KERNEL_SIG_DFL : KernelSighandler = linux_raw_sys :: signal_macros :: SIG_DFL ;
};
}
