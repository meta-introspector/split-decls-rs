// Generated macro for fcntl_setfd (function)
macro_rules! Depcrate_io_fcntlfcntl_setfd {
() => {
// Module: crate::io::fcntl
// Provides: {"fcntl_setfd"}
// Dependencies: {}
# [doc = " `fcntl(fd, F_SETFD, flags)`—Sets a file descriptor's flags."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fcntl.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=fcntl&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/fcntl.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/fcntl.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=fcntl&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/fcntl"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Control-Operations.html#index-fcntl-function"] # [inline] # [doc (alias = "F_SETFD")] pub fn fcntl_setfd < Fd : AsFd > (fd : Fd , flags : FdFlags) -> io :: Result < () > { backend :: io :: syscalls :: fcntl_setfd (fd . as_fd () , flags) }
};
}
