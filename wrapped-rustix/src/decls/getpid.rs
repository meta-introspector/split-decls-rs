macro_rules! deps {
    () => {
        Pid!();
    };
}

macro_rules! getpid {
    () => {
        deps!();
        # [doc = " `getpid()`—Returns the process' ID."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getpid.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getpid.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=getpid&sektion=2"] # [doc = " [illumos]: https://www.illumos.org/man/2/getpid"] # [doc = " [NetBSD]: https://man.netbsd.org/getpid.2"] # [inline] # [must_use] pub fn getpid () -> Pid { backend :: pid :: syscalls :: getpid () }
    };
}

getpid!();