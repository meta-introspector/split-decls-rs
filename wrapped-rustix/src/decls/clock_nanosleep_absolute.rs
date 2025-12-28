macro_rules! deps {
    () => {
        ClockId!();
        Timespec!();
        Result!();
    };
}

macro_rules! clock_nanosleep_absolute {
    () => {
        deps!();
        # [doc = " `clock_nanosleep(id, TIMER_ABSTIME, request, NULL)`—Sleeps until an"] # [doc = " absolute time on a given clock."] # [doc = ""] # [doc = " This is `clock_nanosleep` specialized for the case of an absolute sleep"] # [doc = " interval. See [`clock_nanosleep_relative`] for relative intervals."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_nanosleep.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/clock_nanosleep.2.html"] # [cfg (not (any (apple , target_os = "dragonfly" , target_os = "emscripten" , target_os = "espidf" , target_os = "freebsd" , target_os = "haiku" , target_os = "horizon" , target_os = "openbsd" , target_os = "redox" , target_os = "vita" , target_os = "wasi" ,)))] # [inline] pub fn clock_nanosleep_absolute (id : ClockId , request : & Timespec) -> io :: Result < () > { backend :: thread :: syscalls :: clock_nanosleep_absolute (id , request) }
    };
}

clock_nanosleep_absolute!()