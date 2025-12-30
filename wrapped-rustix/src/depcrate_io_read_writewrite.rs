// Generated macro for write (function)
macro_rules! Depcrate_io_read_writewrite {
() => {
// Module: crate::io::read_write
// Provides: {"write"}
// Dependencies: {}
# [doc = " `write(fd, buf)`—Writes to a stream."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/write.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/write.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/write.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=write&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/write.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/write.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=write&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/write"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/I_002fO-Primitives.html#index-writing-to-a-file-descriptor"] # [inline] pub fn write < Fd : AsFd > (fd : Fd , buf : & [u8]) -> io :: Result < usize > { backend :: io :: syscalls :: write (fd . as_fd () , buf) }
};
}
