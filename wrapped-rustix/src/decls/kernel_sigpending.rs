macro_rules! deps {
    () => {
        KernelSigSet!();
    };
}

macro_rules! kernel_sigpending {
    () => {
        deps!();
        # [doc = " `sigpending()`—Query the pending signals."] # [doc = ""] # [doc = " If this is ever exposed publicly, we should think about whether it should"] # [doc = " mask out signals reserved by libc."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `sigpending`]"] # [doc = ""] # [doc = " [Linux `sigpending`]: https://man7.org/linux/man-pages/man2/sigpending.2.html"] # [inline] pub fn kernel_sigpending () -> KernelSigSet { backend :: runtime :: syscalls :: kernel_sigpending () }
    };
}

kernel_sigpending!()