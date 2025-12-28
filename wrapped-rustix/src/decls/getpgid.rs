macro_rules! deps {
    () => {
        Pid!();
        Result!();
    };
}

macro_rules! getpgid {
    () => {
        deps!();
        # [doc = " `getpgid(pid)`—Returns the process group ID of the given process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getpgid.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getpgid.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=getpgid&sektion=2"] # [doc = " [illumos]: https://www.illumos.org/man/2/getpgid"] # [doc = " [NetBSD]: https://man.netbsd.org/getpgid.2"] # [inline] pub fn getpgid (pid : Option < Pid >) -> io :: Result < Pid > { backend :: process :: syscalls :: getpgid (pid) }
    };
}

getpgid!();