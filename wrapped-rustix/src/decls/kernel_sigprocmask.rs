macro_rules! deps {
    () => {
        Result!();
        KernelSigSet!();
        How!();
    };
}

macro_rules! kernel_sigprocmask {
    () => {
        deps!();
        # [doc = " `rt_sigprocmask(how, set, oldset)`—Adjust the process signal mask."] # [doc = ""] # [doc = " If this is ever exposed publicly, we should think about whether it should"] # [doc = " mask out signals reserved by libc."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If there is a libc in the process, the `set` must not contain any signal"] # [doc = " reserved by the libc."] # [doc = ""] # [doc = " If code elsewhere in the program is depending on delivery of a signal for"] # [doc = " any reason, for example to prevent it from executing some code, this could"] # [doc = " cause it to miss that signal, and for example execute that code. And if the"] # [doc = " caller is depending on delivery of a signal for any reason, its assumptions"] # [doc = " could be broken by code elsewhere in the program calling this function."] # [doc = ""] # [doc = " There may be further safety hazards not yet documented here."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `rt_sigprocmask`]"] # [doc = "  - [Linux `pthread_sigmask`]"] # [doc = ""] # [doc = " [Linux `rt_sigprocmask`]: https://man7.org/linux/man-pages/man2/rt_sigprocmask.2.html"] # [doc = " [Linux `pthread_sigmask`]: https://man7.org/linux/man-pages/man3/pthread_sigmask.3.html"] # [inline] # [doc (alias = "pthread_sigmask")] # [doc (alias = "rt_sigprocmask")] pub unsafe fn kernel_sigprocmask (how : How , set : Option < & KernelSigSet >) -> io :: Result < KernelSigSet > { backend :: runtime :: syscalls :: kernel_sigprocmask (how , set) }
    };
}

kernel_sigprocmask!();