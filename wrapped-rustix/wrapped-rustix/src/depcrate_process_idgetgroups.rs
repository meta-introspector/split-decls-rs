// Generated macro for getgroups (function)
macro_rules! Depcrate_process_idgetgroups {
() => {
// Module: crate::process::id
// Provides: {"getgroups"}
// Dependencies: {}
# [doc = " `getgroups()`—Return a list of the current user's groups."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getgroups.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getgroups.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=getgroups&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/getgroups.2"] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn getgroups () -> io :: Result < Vec < Gid > > { let mut buffer = Vec :: with_capacity (0) ; let ngroups = backend :: process :: syscalls :: getgroups (& mut buffer) ? ; buffer . resize (ngroups , Gid :: ROOT) ; backend :: process :: syscalls :: getgroups (& mut buffer) ? ; Ok (buffer) }
};
}
