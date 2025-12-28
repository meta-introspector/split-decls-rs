macro_rules! deps {
    () => {
        Timespec!();
        ClockId!();
    };
}

macro_rules! clock_gettime {
    () => {
        deps!();
        # [doc = " `clock_gettime(id)`—Returns the current value of a clock."] # [doc = ""] # [doc = " This function uses `ClockId` which only contains clocks which are known to"] # [doc = " always be supported at runtime, allowing this function to be infallible."] # [doc = " For a greater set of clocks and dynamic clock support, see"] # [doc = " [`clock_gettime_dynamic`]."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_gettime.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/clock_gettime.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=clock_getres&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/clock_getres.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/clock_getres.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=clock_getres&section=2"] # [doc = " [illumos]: https://illumos.org/man/3C/clock_gettime"] # [cfg (not (target_os = "wasi"))] # [inline] # [must_use] pub fn clock_gettime (id : ClockId) -> Timespec { backend :: time :: syscalls :: clock_gettime (id) }
    };
}

clock_gettime!()