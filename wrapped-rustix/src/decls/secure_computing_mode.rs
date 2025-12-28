macro_rules! deps {
    () => {
        Result!();
        SecureComputingMode!();
    };
}

macro_rules! secure_computing_mode {
    () => {
        deps!();
        # [doc = " Get the secure computing mode of the calling thread."] # [doc = ""] # [doc = " If the caller is not in secure computing mode, this returns"] # [doc = " [`SecureComputingMode::Disabled`]. If the caller is in strict secure"] # [doc = " computing mode, then this call will cause a [`Signal::KILL`] signal to be"] # [doc = " sent to the process. If the caller is in filter mode, and this system call"] # [doc = " is allowed by the seccomp filters, it returns"] # [doc = " [`SecureComputingMode::Filter`]; otherwise, the process is killed with a"] # [doc = " [`Signal::KILL`] signal."] # [doc = ""] # [doc = " Since Linux 3.8, the Seccomp field of the `/proc/[pid]/status` file"] # [doc = " provides a method of obtaining the same information, without the risk that"] # [doc = " the process is killed; see [the `proc` manual page]."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_SECCOMP,…)`]"] # [doc = ""] # [doc = " [`Signal::KILL`]: crate::signal::Signal::KILL"] # [doc = " [`prctl(PR_GET_SECCOMP,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [doc = " [the `proc` manual page]: https://man7.org/linux/man-pages/man5/proc.5.html"] # [inline] pub fn secure_computing_mode () -> io :: Result < SecureComputingMode > { unsafe { prctl_1arg (PR_GET_SECCOMP) } . and_then (TryInto :: try_into) }
    };
}

secure_computing_mode!();