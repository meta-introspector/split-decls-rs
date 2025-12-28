macro_rules! deps {
    () => {
        WaitStatus!();
        Pid!();
        Result!();
    };
}

macro_rules! waitpgid {
    () => {
        deps!();
        # [doc = " `waitpid(-pgid, waitopts)`—Wait for a process in a specific process group"] # [doc = " to change state."] # [doc = ""] # [doc = " The call will wait for any child process with the given pgid."] # [doc = ""] # [doc = " On Success, returns the status of the selected process."] # [doc = ""] # [doc = " If `NOHANG` was specified in the options, and no selected child process"] # [doc = " changed state, returns `None`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/wait.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/waitpid.2.html"] # [cfg (not (target_os = "wasi"))] # [inline] pub fn waitpgid (pgid : Pid , waitopts : WaitOptions) -> io :: Result < Option < (Pid , WaitStatus) > > { backend :: process :: syscalls :: waitpgid (pgid , waitopts) }
    };
}

waitpgid!()