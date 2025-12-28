macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! grantpt {
    () => {
        deps!();
        # [doc = " `grantpt(fd)`—Grant access to the user side of a pseudoterminal."] # [doc = ""] # [doc = " On Linux, calling this function has no effect, as the kernel is expected to"] # [doc = " grant the appropriate access. On all other platforms, this function has"] # [doc = " unspecified behavior if the calling process has a [`Signal::CHILD`] signal"] # [doc = " handler installed."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/grantpt.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/grantpt.3.html"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Allocation.html#index-grantpt"] # [doc = " [`Signal::CHILD`]: crate::process::Signal::CHILD"] # [inline] pub fn grantpt < Fd : AsFd > (fd : Fd) -> io :: Result < () > { # [cfg (not (linux_kernel))] { backend :: pty :: syscalls :: grantpt (fd . as_fd ()) } # [cfg (linux_kernel)] { let _ = fd ; Ok (()) } }
    };
}

grantpt!();