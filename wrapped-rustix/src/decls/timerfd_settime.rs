macro_rules! deps {
    () => {
        Result!();
        Itimerspec!();
    };
}

macro_rules! timerfd_settime {
    () => {
        deps!();
        # [doc = " `timerfd_settime(clockid, flags, new_value)`—Set the time on a timer."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/timerfd_settime.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=timerfd_settime&sektion=2"] # [doc = " [illumos]: https://illumos.org/man/3C/timerfd_settime"] # [doc = " [NetBSD]: https://man.netbsd.org/timerfd_settime.2"] # [inline] pub fn timerfd_settime < Fd : AsFd > (fd : Fd , flags : TimerfdTimerFlags , new_value : & Itimerspec ,) -> io :: Result < Itimerspec > { backend :: time :: syscalls :: timerfd_settime (fd . as_fd () , flags , new_value) }
    };
}

timerfd_settime!();