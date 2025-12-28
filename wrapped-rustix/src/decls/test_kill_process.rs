macro_rules! deps {
    () => {
        Result!();
        Pid!();
    };
}

macro_rules! test_kill_process {
    () => {
        deps!();
        # [doc = " `kill(pid, 0)`—Check validity of pid and permissions to send signals to"] # [doc = " the process, without actually sending any signals."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/kill.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html"] # [inline] # [doc (alias = "kill")] pub fn test_kill_process (pid : Pid) -> io :: Result < () > { backend :: process :: syscalls :: test_kill_process (pid) }
    };
}

test_kill_process!();