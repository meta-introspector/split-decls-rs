// Generated macro for setpgid (function)
macro_rules! Depcrate_process_idsetpgid {
() => {
// Module: crate::process::id
// Provides: {"setpgid"}
// Dependencies: {}
# [doc = " `setpgid(pid, pgid)`—Sets the process group ID of the given process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = "  - [NetBSD]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/setpgid.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setpgid.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=setpgid&sektion=2"] # [doc = " [illumos]: https://www.illumos.org/man/2/setpgid"] # [doc = " [NetBSD]: https://man.netbsd.org/setpgid.2"] # [inline] pub fn setpgid (pid : Option < Pid > , pgid : Option < Pid >) -> io :: Result < () > { backend :: process :: syscalls :: setpgid (pid , pgid) }
};
}
