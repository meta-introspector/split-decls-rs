macro_rules! deps {
    () => {
        Timespec!();
        NanosleepRelativeResult!();
    };
}

macro_rules! nanosleep {
    () => {
        deps!();
        # [doc = " `nanosleep(request, remain)`—Sleeps for a duration."] # [doc = ""] # [doc = " This effectively uses the system monotonic clock."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/nanosleep.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/nanosleep.2.html"] # [inline] pub fn nanosleep (request : & Timespec) -> NanosleepRelativeResult { backend :: thread :: syscalls :: nanosleep (request) }
    };
}

nanosleep!()