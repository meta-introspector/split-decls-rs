macro_rules! deps {
    () => {
        KernelSighandler!();
    };
}

macro_rules! KERNEL_SIG_DFL {
    () => {
        deps!();
        # [doc = " A special “default” signal handler representing the default behavior"] # [doc = " for handling a signal."] # [doc = ""] # [doc = " If you're looking for `KERNEL_SIG_IGN`; use [`kernel_sig_ign`]."] # [doc (alias = "SIG_DFL")] pub const KERNEL_SIG_DFL : KernelSighandler = linux_raw_sys :: signal_macros :: SIG_DFL ;
    };
}

KERNEL_SIG_DFL!();