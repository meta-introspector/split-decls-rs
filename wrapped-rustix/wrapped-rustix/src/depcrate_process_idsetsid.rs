// Generated macro for setsid (function)
macro_rules! Depcrate_process_idsetsid {
() => {
// Module: crate::process::id
// Provides: {"setsid"}
// Dependencies: {}
# [doc = " `setsid()`—Create a new session."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/setsid.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setsid.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=setsid&sektion=2"] # [doc = " [illumos]: https://www.illumos.org/man/2/setsid"] # [doc = " [NetBSD]: https://man.netbsd.org/setsid.2"] # [inline] pub fn setsid () -> io :: Result < Pid > { backend :: process :: syscalls :: setsid () }
};
}
