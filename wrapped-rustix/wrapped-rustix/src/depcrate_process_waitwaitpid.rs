// Generated macro for waitpid (function)
macro_rules! Depcrate_process_waitwaitpid {
() => {
// Module: crate::process::wait
// Provides: {"waitpid"}
// Dependencies: {}
# [doc = " `waitpid(pid, waitopts)`—Wait for a specific process to change state."] # [doc = ""] # [doc = " If the pid is `None`, the call will wait for any child process whose"] # [doc = " process group id matches that of the calling process. Otherwise, the call"] # [doc = " will wait for the child process with the given pid."] # [doc = ""] # [doc = " On Success, returns the status of the selected process."] # [doc = ""] # [doc = " If `NOHANG` was specified in the options, and the selected child process"] # [doc = " didn't change state, returns `None`."] # [doc = ""] # [doc = " To wait for a given process group (the `< -1` case of `waitpid`), use"] # [doc = " [`waitpgid`] or [`waitid`]. To wait for any process (the `-1` case of"] # [doc = " `waitpid`), use [`wait`]."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/wait.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/waitpid.2.html"] # [doc (alias = "wait4")] # [cfg (not (target_os = "wasi"))] # [inline] pub fn waitpid (pid : Option < Pid > , waitopts : WaitOptions) -> io :: Result < Option < (Pid , WaitStatus) > > { backend :: process :: syscalls :: waitpid (pid , waitopts) }
};
}
