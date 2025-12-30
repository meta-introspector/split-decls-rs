// Generated macro for wait (function)
macro_rules! Depcrate_process_waitwait {
() => {
// Module: crate::process::wait
// Provides: {"wait"}
// Dependencies: {}
# [doc = " `wait(waitopts)`—Wait for any of the children of calling process to"] # [doc = " change state."] # [doc = ""] # [doc = " On success, returns the pid of the child process whose state changed, and"] # [doc = " the status of said process."] # [doc = ""] # [doc = " If `NOHANG` was specified in the options, and the selected child process"] # [doc = " didn't change state, returns `None`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/wait.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/waitpid.2.html"] # [cfg (not (target_os = "wasi"))] # [inline] pub fn wait (waitopts : WaitOptions) -> io :: Result < Option < (Pid , WaitStatus) > > { backend :: process :: syscalls :: wait (waitopts) }
};
}
