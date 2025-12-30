// Generated macro for kill_current_process_group (function)
macro_rules! Depcrate_process_killkill_current_process_group {
() => {
// Module: crate::process::kill
// Provides: {"kill_current_process_group"}
// Dependencies: {}
# [doc = " `kill(0, sig)`—Sends a signal to all processes in the current process"] # [doc = " group."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/kill.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html"] # [inline] # [doc (alias = "kill")] pub fn kill_current_process_group (sig : Signal) -> io :: Result < () > { backend :: process :: syscalls :: kill_current_process_group (sig) }
};
}
