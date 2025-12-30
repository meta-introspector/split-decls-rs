// Generated macro for tcsetattr (function)
macro_rules! Depcrate_termios_tctcsetattr {
() => {
// Module: crate::termios::tc
// Provides: {"tcsetattr"}
// Dependencies: {}
# [doc = " `tcsetattr(fd)`—Set terminal attributes."] # [doc = ""] # [doc = " Also known as the `TCSETS` (or `TCSETS2` on Linux) operation with `ioctl`."] # [doc = ""] # [doc = " On Linux, this uses `TCSETS2`. If that fails in a way that indicates that"] # [doc = " the host doesn't support it, this falls back to the old `TCSETS`, and fails"] # [doc = " with `io::Errno::RANGE` if the input or output speeds cannot be supported."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX `tcsetattr`]"] # [doc = "  - [Linux `ioctl_tty`]"] # [doc = "  - [Linux `termios`]"] # [doc = ""] # [doc = " [POSIX `tcsetattr`]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/tcsetattr.html"] # [doc = " [Linux `ioctl_tty`]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html"] # [doc = " [Linux `termios`]: https://man7.org/linux/man-pages/man3/termios.3.html"] # [cfg (not (target_os = "espidf"))] # [inline] # [doc (alias = "TCSETS")] # [doc (alias = "TCSETS2")] # [doc (alias = "tcsetattr2")] pub fn tcsetattr < Fd : AsFd > (fd : Fd , optional_actions : OptionalActions , termios : & Termios ,) -> io :: Result < () > { backend :: termios :: syscalls :: tcsetattr (fd . as_fd () , optional_actions , termios) }
};
}
