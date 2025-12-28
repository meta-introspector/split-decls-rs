macro_rules! deps {
    () => {
        Uid!();
    };
}

macro_rules! geteuid {
    () => {
        deps!();
        # [doc = " `geteuid()`—Returns the process' effective user ID."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/geteuid.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/geteuid.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=geteuid&sektion=2"] # [doc = " [illumos]: https://www.illumos.org/man/2/geteuid"] # [doc = " [NetBSD]: https://man.netbsd.org/geteuid.2"] # [inline] # [must_use] pub fn geteuid () -> Uid { backend :: ugid :: syscalls :: geteuid () }
    };
}

geteuid!();