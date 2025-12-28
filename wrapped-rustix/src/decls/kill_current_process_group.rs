macro_rules! deps {
    () => {
        Result!();
        Signal!();
    };
}

macro_rules! kill_current_process_group {
    () => {
        deps!();
        # [doc = " `kill(0, sig)`—Sends a signal to all processes in the current process"] # [doc = " group."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/kill.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html"] # [inline] # [doc (alias = "kill")] pub fn kill_current_process_group (sig : Signal) -> io :: Result < () > { backend :: process :: syscalls :: kill_current_process_group (sig) }
    };
}

kill_current_process_group!();