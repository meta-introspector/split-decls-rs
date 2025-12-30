// Generated macro for getsid (function)
macro_rules! Depcrate_process_idgetsid {
() => {
// Module: crate::process::id
// Provides: {"getsid"}
// Dependencies: {}
# [doc = " `getsid(pid)`—Get the session ID of the given process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getsid.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getsid.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=getsid&sektion=2"] # [doc = " [illumos]: https://www.illumos.org/man/2/getsid"] # [doc = " [NetBSD]: https://man.netbsd.org/getsid.2"] # [cfg (not (target_os = "redox"))] # [inline] pub fn getsid (pid : Option < Pid >) -> io :: Result < Pid > { backend :: process :: syscalls :: getsid (pid) }
};
}
