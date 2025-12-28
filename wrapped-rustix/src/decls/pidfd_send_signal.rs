macro_rules! deps {
    () => {
        Signal!();
        Result!();
    };
}

macro_rules! pidfd_send_signal {
    () => {
        deps!();
        # [doc = " `syscall(SYS_pidfd_send_signal, pidfd, sig, NULL, 0)`—Send a signal to a"] # [doc = " process specified by a file descriptor."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/pidfd_send_signal.2.html"] # [inline] pub fn pidfd_send_signal < Fd : AsFd > (pidfd : Fd , sig : Signal) -> io :: Result < () > { backend :: process :: syscalls :: pidfd_send_signal (pidfd . as_fd () , sig) }
    };
}

pidfd_send_signal!();