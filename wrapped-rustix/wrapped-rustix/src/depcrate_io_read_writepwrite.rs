// Generated macro for pwrite (function)
macro_rules! Depcrate_io_read_writepwrite {
() => {
// Module: crate::io::read_write
// Provides: {"pwrite"}
// Dependencies: {}
# [doc = " `pwrite(fd, bufs)`—Writes to a file at a given position."] # [doc = ""] # [doc = " Contrary to POSIX, on many popular platforms including Linux and FreeBSD,"] # [doc = " if the file is opened in append mode, this ignores the offset appends the"] # [doc = " data to the end of the file."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/pwrite.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/pwrite.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pwrite.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=pwrite&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/pwrite.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/pwrite.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=pwrite&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/pwrite"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/I_002fO-Primitives.html#index-pwrite64"] # [cfg (not (windows))] # [inline] pub fn pwrite < Fd : AsFd > (fd : Fd , buf : & [u8] , offset : u64) -> io :: Result < usize > { backend :: io :: syscalls :: pwrite (fd . as_fd () , buf , offset) }
};
}
