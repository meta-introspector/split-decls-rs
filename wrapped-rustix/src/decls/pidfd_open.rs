macro_rules! deps {
    () => {
        Pid!();
        Result!();
    };
}

macro_rules! pidfd_open {
    () => {
        deps!();
        # [doc = " `syscall(SYS_pidfd_open, pid, flags)`—Creates a file descriptor for a"] # [doc = " process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/pidfd_open.2.html"] # [inline] pub fn pidfd_open (pid : Pid , flags : PidfdFlags) -> io :: Result < OwnedFd > { backend :: process :: syscalls :: pidfd_open (pid , flags) }
    };
}

pidfd_open!()