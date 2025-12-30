// Generated macro for writev (function)
macro_rules! Depcrate_io_read_writewritev {
() => {
// Module: crate::io::read_write
// Provides: {"writev"}
// Dependencies: {}
# [doc = " `writev(fd, bufs)`—Writes to a stream from multiple buffers."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/writev.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/writev.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/writev.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=writev&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/writev.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/writev.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=writev&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/writev"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Scatter_002dGather.html#index-writev"] # [cfg (not (any (windows , target_os = "espidf" , target_os = "horizon")))] # [inline] pub fn writev < Fd : AsFd > (fd : Fd , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { backend :: io :: syscalls :: writev (fd . as_fd () , bufs) }
};
}
