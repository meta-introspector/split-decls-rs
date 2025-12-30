// Generated macro for tcsetwinsize (function)
macro_rules! Depcrate_termios_tctcsetwinsize {
() => {
// Module: crate::termios::tc
// Provides: {"tcsetwinsize"}
// Dependencies: {}
# [doc = " `tcsetwinsize(fd)`—Set the current terminal window size."] # [doc = ""] # [doc = " Also known as the `TIOCSWINSZ` operation with `ioctl`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon")))] # [inline] # [doc (alias = "TIOCSWINSZ")] pub fn tcsetwinsize < Fd : AsFd > (fd : Fd , winsize : Winsize) -> io :: Result < () > { backend :: termios :: syscalls :: tcsetwinsize (fd . as_fd () , winsize) }
};
}
