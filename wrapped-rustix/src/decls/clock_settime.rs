macro_rules! deps {
    () => {
        ClockId!();
        Result!();
        Timespec!();
    };
}

macro_rules! clock_settime {
    () => {
        deps!();
        # [doc = " `clock_settime(id, timespec)`—Sets the current value of a settable clock."] # [doc = ""] # [doc = " This fails with [`io::Errno::INVAL`] if the clock is not settable, and"] # [doc = " [`io::Errno::ACCESS`] if the current process does not have permission to"] # [doc = " set it."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_settime.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/clock_settime.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=clock_settime&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/clock_settime.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/clock_settime.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=clock_settime&section=2"] # [doc = " [illumos]: https://illumos.org/man/3C/clock_settime"] # [cfg (not (any (target_os = "redox" , target_os = "wasi" , all (apple , not (target_os = "macos")))))] # [inline] pub fn clock_settime (id : ClockId , timespec : Timespec) -> io :: Result < () > { backend :: time :: syscalls :: clock_settime (id , timespec) }
    };
}

clock_settime!()