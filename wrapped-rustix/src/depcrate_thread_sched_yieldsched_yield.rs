// Generated macro for sched_yield (function)
macro_rules! Depcrate_thread_sched_yieldsched_yield {
() => {
// Module: crate::thread::sched_yield
// Provides: {"sched_yield"}
// Dependencies: {}
# [doc = " `sched_yield()`—Hints to the OS that other processes should run."] # [doc = ""] # [doc = " This function always succeeds."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/sched_yield.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/sched_yield.2.html"] # [inline] pub fn sched_yield () { backend :: thread :: syscalls :: sched_yield () }
};
}
