macro_rules! deps {
    () => {
        Result!();
        Signal!();
        KernelSigaction!();
    };
}

macro_rules! kernel_sigaction {
    () => {
        deps!();
        # [doc = " `sigaction(signal, &new, &old)`—Modify and/or query a signal handler."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " You're on your own. And on top of all the troubles with signal handlers,"] # [doc = " this implementation is highly experimental. Even further, it differs from"] # [doc = " the libc `sigaction` in several non-obvious and unsafe ways."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/sigaction.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/sigaction.2.html"] # [inline] pub unsafe fn kernel_sigaction (signal : Signal , new : Option < KernelSigaction > ,) -> io :: Result < KernelSigaction > { backend :: runtime :: syscalls :: kernel_sigaction (signal , new) }
    };
}

kernel_sigaction!();