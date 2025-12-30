// Generated macro for getegid (function)
macro_rules! Depcrate_process_idgetegid {
() => {
// Module: crate::process::id
// Provides: {"getegid"}
// Dependencies: {}
# [doc = " `getegid()`—Returns the process' effective group ID."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getegid.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getegid.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=getegid&sektion=2"] # [doc = " [illumos]: https://www.illumos.org/man/2/getegid"] # [doc = " [NetBSD]: https://man.netbsd.org/getegid.2"] # [inline] # [must_use] pub fn getegid () -> Gid { backend :: ugid :: syscalls :: getegid () }
};
}
