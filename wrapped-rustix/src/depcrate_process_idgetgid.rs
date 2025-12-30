// Generated macro for getgid (function)
macro_rules! Depcrate_process_idgetgid {
() => {
// Module: crate::process::id
// Provides: {"getgid"}
// Dependencies: {}
# [doc = " `getgid()`—Returns the process' real group ID."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getgid.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getgid.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=getgid&sektion=2"] # [doc = " [illumos]: https://www.illumos.org/man/2/getgid"] # [doc = " [NetBSD]: https://man.netbsd.org/getgid.2"] # [inline] # [must_use] pub fn getgid () -> Gid { backend :: ugid :: syscalls :: getgid () }
};
}
