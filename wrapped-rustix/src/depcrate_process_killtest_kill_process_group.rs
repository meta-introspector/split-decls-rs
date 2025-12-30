// Generated macro for test_kill_process_group (function)
macro_rules! Depcrate_process_killtest_kill_process_group {
() => {
// Module: crate::process::kill
// Provides: {"test_kill_process_group"}
// Dependencies: {}
# [doc = " `kill(-pid, 0)`—Check validity of pid and permissions to send signals to"] # [doc = " all processes in the process group, without actually sending any signals."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/kill.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html"] # [inline] # [doc (alias = "kill")] pub fn test_kill_process_group (pid : Pid) -> io :: Result < () > { backend :: process :: syscalls :: test_kill_process_group (pid) }
};
}
