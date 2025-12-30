// Generated macro for tcgetsid (function)
macro_rules! Depcrate_termios_tctcgetsid {
() => {
// Module: crate::termios::tc
// Provides: {"tcgetsid"}
// Dependencies: {}
# [doc = " `tcgetsid(fd)`—Return the session ID of the current session with `fd` as"] # [doc = " its controlling terminal."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/tcgetsid.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/tcgetsid.3.html"] # [inline] # [doc (alias = "TIOCGSID")] pub fn tcgetsid < Fd : AsFd > (fd : Fd) -> io :: Result < Pid > { backend :: termios :: syscalls :: tcgetsid (fd . as_fd ()) }
};
}
