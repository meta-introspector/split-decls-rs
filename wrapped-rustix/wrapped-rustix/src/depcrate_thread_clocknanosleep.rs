// Generated macro for nanosleep (function)
macro_rules! Depcrate_thread_clocknanosleep {
() => {
// Module: crate::thread::clock
// Provides: {"nanosleep"}
// Dependencies: {}
# [doc = " `nanosleep(request, remain)`—Sleeps for a duration."] # [doc = ""] # [doc = " This effectively uses the system monotonic clock."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/nanosleep.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/nanosleep.2.html"] # [inline] pub fn nanosleep (request : & Timespec) -> NanosleepRelativeResult { backend :: thread :: syscalls :: nanosleep (request) }
};
}
