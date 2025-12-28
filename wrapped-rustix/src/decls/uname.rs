macro_rules! deps {
    () => {
        Uname!();
    };
}

macro_rules! uname {
    () => {
        deps!();
        # [doc = " `uname()`—Returns high-level information about the runtime OS and"] # [doc = " hardware."] # [doc = ""] # [doc = " For `gethostname()`, use [`Uname::nodename`] on the result."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [NetBSD]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/uname.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/uname.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/uname.3.html"] # [doc = " [NetBSD]: https://man.netbsd.org/uname.3"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=uname&sektion=3"] # [doc = " [OpenBSD]: https://man.openbsd.org/uname.3"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=uname&section=3"] # [doc = " [illumos]: https://illumos.org/man/2/uname"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Platform-Type.html"] # [doc (alias = "gethostname")] # [inline] pub fn uname () -> Uname { Uname (backend :: system :: syscalls :: uname ()) }
    };
}

uname!()