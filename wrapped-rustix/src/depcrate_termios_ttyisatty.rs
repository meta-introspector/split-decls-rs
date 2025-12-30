// Generated macro for isatty (function)
macro_rules! Depcrate_termios_ttyisatty {
() => {
// Module: crate::termios::tty
// Provides: {"isatty"}
// Dependencies: {}
# [doc = " `isatty(fd)`—Tests whether a file descriptor refers to a terminal."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/isatty.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/isatty.3.html"] # [inline] pub fn isatty < Fd : AsFd > (fd : Fd) -> bool { backend :: termios :: syscalls :: isatty (fd . as_fd ()) }
};
}
