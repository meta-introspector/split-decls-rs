macro_rules! deps {
    () => {
        ClockId!();
        Timespec!();
    };
}

macro_rules! clock_getres {
    () => {
        deps!();
        # [doc = " `clock_getres(id)`—Returns the resolution of a clock."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_getres.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/clock_getres.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=clock_getres&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/clock_getres.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/clock_getres.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=clock_getres&section=2"] # [doc = " [illumos]: https://illumos.org/man/3C/clock_getres"] # [cfg (not (any (target_os = "redox" , target_os = "wasi")))] # [inline] # [must_use] pub fn clock_getres (id : ClockId) -> Timespec { backend :: time :: syscalls :: clock_getres (id) }
    };
}

clock_getres!();