macro_rules! deps {
    () => {
        Pid!();
    };
}

macro_rules! getppid {
    () => {
        deps!();
        # [doc = " `getppid()`—Returns the parent process' ID."] # [doc = ""] # [doc = " This will return `None` if the current process has no parent (or no parent"] # [doc = " accessible in the current PID namespace), such as if the current process is"] # [doc = " the init process ([`Pid::INIT`])."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getppid.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getppid.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=getppid&sektion=2"] # [doc = " [illumos]: https://www.illumos.org/man/2/getppid"] # [doc = " [NetBSD]: https://man.netbsd.org/getppid.2"] # [inline] # [must_use] pub fn getppid () -> Option < Pid > { backend :: process :: syscalls :: getppid () }
    };
}

getppid!();