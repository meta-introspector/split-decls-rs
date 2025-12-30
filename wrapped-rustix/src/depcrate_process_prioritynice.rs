// Generated macro for nice (function)
macro_rules! Depcrate_process_prioritynice {
() => {
// Module: crate::process::priority
// Provides: {"nice"}
// Dependencies: {}
# [doc = " `nice(inc)`—Adjust the scheduling priority of the current process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/nice.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/nice.2.html"] # [inline] pub fn nice (inc : i32) -> io :: Result < i32 > { backend :: process :: syscalls :: nice (inc) }
};
}
