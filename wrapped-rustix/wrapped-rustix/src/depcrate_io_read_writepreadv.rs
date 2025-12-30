// Generated macro for preadv (function)
macro_rules! Depcrate_io_read_writepreadv {
() => {
// Module: crate::io::read_write
// Provides: {"preadv"}
// Dependencies: {}
# [doc = " `preadv(fd, bufs, offset)`—Reads from a file at a given position into"] # [doc = " multiple buffers."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/preadv.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=preadv&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/preadv.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/preadv.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=preadv&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/preadv"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Scatter_002dGather.html#index-preadv64"] # [cfg (not (any (windows , target_os = "cygwin" , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "nto" , target_os = "redox" , target_os = "solaris" , target_os = "vita" ,)))] # [inline] pub fn preadv < Fd : AsFd > (fd : Fd , bufs : & mut [IoSliceMut < '_ >] , offset : u64) -> io :: Result < usize > { backend :: io :: syscalls :: preadv (fd . as_fd () , bufs , offset) }
};
}
