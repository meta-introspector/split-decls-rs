macro_rules! deps {
    () => {
        KernelSighandler!();
    };
}

macro_rules! kernel_sig_ign {
    () => {
        deps!();
        # [doc = " Return a special “ignore” signal handler for ignoring signals."] # [doc = ""] # [doc = " This isn't the `SIG_IGN` value itself; it's a function that returns the"] # [doc = " `SIG_IGN` value."] # [doc = ""] # [doc = " If you're looking for `kernel_sig_dfl`; use [`KERNEL_SIG_DFL`]."] # [doc (alias = "SIG_IGN")] # [must_use] pub const fn kernel_sig_ign () -> KernelSighandler { linux_raw_sys :: signal_macros :: sig_ign () }
    };
}

kernel_sig_ign!();