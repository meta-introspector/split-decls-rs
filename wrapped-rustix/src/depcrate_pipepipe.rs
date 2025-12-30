// Generated macro for pipe (function)
macro_rules! Depcrate_pipepipe {
() => {
// Module: crate::pipe
// Provides: {"pipe"}
// Dependencies: {}
# [doc = " `pipe()`—Creates a pipe."] # [doc = ""] # [doc = " This function creates a pipe and returns two file descriptors, for the"] # [doc = " reading and writing ends of the pipe, respectively."] # [doc = ""] # [doc = " See [`pipe_with`] to pass additional flags."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/pipe.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/pipe.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pipe.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=pipe&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/pipe.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/pipe.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=pipe&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/pipe"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Creating-a-Pipe.html"] # [inline] pub fn pipe () -> io :: Result < (OwnedFd , OwnedFd) > { backend :: pipe :: syscalls :: pipe () }
};
}
