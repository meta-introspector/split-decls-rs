macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! test_kill_current_process_group {
    () => {
        deps!();
        # [doc = " `kill(0, 0)`—Check validity of pid and permissions to send signals to the"] # [doc = " all processes in the current process group, without actually sending any"] # [doc = " signals."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/kill.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html"] # [inline] # [doc (alias = "kill")] pub fn test_kill_current_process_group () -> io :: Result < () > { backend :: process :: syscalls :: test_kill_current_process_group () }
    };
}

test_kill_current_process_group!()