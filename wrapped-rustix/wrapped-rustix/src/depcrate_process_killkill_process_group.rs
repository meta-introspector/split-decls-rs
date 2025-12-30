// Generated macro for kill_process_group (function)
macro_rules! Depcrate_process_killkill_process_group {
() => {
// Module: crate::process::kill
// Provides: {"kill_process_group"}
// Dependencies: {}
# [doc = " `kill(-pid, sig)`—Sends a signal to all processes in a process group."] # [doc = ""] # [doc = " If `pid` is [`Pid::INIT`], this sends a signal to all processes the current"] # [doc = " process has permission to send signals to, except process `Pid::INIT`,"] # [doc = " possibly other system-specific processes, and on some systems, the current"] # [doc = " process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/kill.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html"] # [inline] # [doc (alias = "kill")] pub fn kill_process_group (pid : Pid , sig : Signal) -> io :: Result < () > { backend :: process :: syscalls :: kill_process_group (pid , sig) }
};
}
