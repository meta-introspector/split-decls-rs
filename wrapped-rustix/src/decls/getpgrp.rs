macro_rules! deps {
    () => {
        Pid!();
    };
}

macro_rules! getpgrp {
    () => {
        deps!();
        # [doc = " `getpgrp()`—Returns the process' group ID."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getpgrp.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getpgrp.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=getpgrp&sektion=2"] # [doc = " [illumos]: https://www.illumos.org/man/2/getpgrp"] # [doc = " [NetBSD]: https://man.netbsd.org/getpgrp.2"] # [inline] # [must_use] pub fn getpgrp () -> Pid { backend :: process :: syscalls :: getpgrp () }
    };
}

getpgrp!()