macro_rules! deps {
    () => {
        Uid!();
    };
}

macro_rules! getuid {
    () => {
        deps!();
        # [doc = " `getuid()`—Returns the process' real user ID."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getuid.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getuid.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=getuid&sektion=2"] # [doc = " [illumos]: https://www.illumos.org/man/2/getuid"] # [doc = " [NetBSD]: https://man.netbsd.org/getuid.2"] # [inline] # [must_use] pub fn getuid () -> Uid { backend :: ugid :: syscalls :: getuid () }
    };
}

getuid!()