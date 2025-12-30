// Generated macro for tcdrain (function)
macro_rules! Depcrate_termios_tctcdrain {
() => {
// Module: crate::termios::tc
// Provides: {"tcdrain"}
// Dependencies: {}
# [doc = " `tcdrain(fd, duration)`—Wait until all pending output has been written."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX `tcdrain`]"] # [doc = "  - [Linux `ioctl_tty`]"] # [doc = "  - [Linux `termios`]"] # [doc = ""] # [doc = " [POSIX `tcsetattr`]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/tcdrain.html"] # [doc = " [Linux `ioctl_tty`]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html"] # [doc = " [Linux `termios`]: https://man7.org/linux/man-pages/man3/termios.3.html"] # [cfg (not (target_os = "espidf"))] # [inline] pub fn tcdrain < Fd : AsFd > (fd : Fd) -> io :: Result < () > { backend :: termios :: syscalls :: tcdrain (fd . as_fd ()) }
};
}
