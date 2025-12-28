macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! timerfd_create {
    () => {
        deps!();
        # [doc = " `timerfd_create(clockid, flags)`—Create a timer."] # [doc = ""] # [doc = " For a higher-level API to timerfd functionality, see the [timerfd] crate."] # [doc = ""] # [doc = " [timerfd]: https://crates.io/crates/timerfd"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/timerfd_create.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=timerfd_create&sektion=2"] # [doc = " [illumos]: https://illumos.org/man/3C/timerfd_create"] # [doc = " [NetBSD]: https://man.netbsd.org/timerfd_create.2"] # [inline] pub fn timerfd_create (clockid : TimerfdClockId , flags : TimerfdFlags) -> io :: Result < OwnedFd > { backend :: time :: syscalls :: timerfd_create (clockid , flags) }
    };
}

timerfd_create!();