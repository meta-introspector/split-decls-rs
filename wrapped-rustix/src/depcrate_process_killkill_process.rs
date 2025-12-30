// Generated macro for kill_process (function)
macro_rules! Depcrate_process_killkill_process {
() => {
// Module: crate::process::kill
// Provides: {"kill_process"}
// Dependencies: {}
# [doc = " `kill(pid, sig)`—Sends a signal to a process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/kill.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html"] # [inline] # [doc (alias = "kill")] pub fn kill_process (pid : Pid , sig : Signal) -> io :: Result < () > { backend :: process :: syscalls :: kill_process (pid , sig) }
};
}
