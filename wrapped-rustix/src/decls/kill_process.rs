macro_rules! deps {
    () => {
        Signal!();
        Pid!();
        Result!();
    };
}

macro_rules! kill_process {
    () => {
        deps!();
        # [doc = " `kill(pid, sig)`—Sends a signal to a process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/kill.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html"] # [inline] # [doc (alias = "kill")] pub fn kill_process (pid : Pid , sig : Signal) -> io :: Result < () > { backend :: process :: syscalls :: kill_process (pid , sig) }
    };
}

kill_process!();