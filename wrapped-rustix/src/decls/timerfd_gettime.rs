macro_rules! deps {
    () => {
        Itimerspec!();
        Result!();
    };
}

macro_rules! timerfd_gettime {
    () => {
        deps!();
        # [doc = " `timerfd_gettime(clockid, flags)`—Query a timer."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/timerfd_gettime.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=timerfd_gettime&sektion=2"] # [doc = " [illumos]: https://illumos.org/man/3C/timerfd_gettime"] # [doc = " [NetBSD]: https://man.netbsd.org/timerfd_gettime.2"] # [inline] pub fn timerfd_gettime < Fd : AsFd > (fd : Fd) -> io :: Result < Itimerspec > { backend :: time :: syscalls :: timerfd_gettime (fd . as_fd ()) }
    };
}

timerfd_gettime!();