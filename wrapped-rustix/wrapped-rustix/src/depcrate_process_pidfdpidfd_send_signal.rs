// Generated macro for pidfd_send_signal (function)
macro_rules! Depcrate_process_pidfdpidfd_send_signal {
() => {
// Module: crate::process::pidfd
// Provides: {"pidfd_send_signal"}
// Dependencies: {}
# [doc = " `syscall(SYS_pidfd_send_signal, pidfd, sig, NULL, 0)`—Send a signal to a"] # [doc = " process specified by a file descriptor."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/pidfd_send_signal.2.html"] # [inline] pub fn pidfd_send_signal < Fd : AsFd > (pidfd : Fd , sig : Signal) -> io :: Result < () > { backend :: process :: syscalls :: pidfd_send_signal (pidfd . as_fd () , sig) }
};
}
