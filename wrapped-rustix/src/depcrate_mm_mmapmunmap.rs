// Generated macro for munmap (function)
macro_rules! Depcrate_mm_mmapmunmap {
() => {
// Module: crate::mm::mmap
// Provides: {"munmap"}
// Dependencies: {}
# [doc = " `munmap(ptr, len)`—Remove a memory mapping."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `ptr` must be aligned to the applicable page size, and the range of memory"] # [doc = " starting at `ptr` and extending for `len` bytes, rounded up to the"] # [doc = " applicable page size, must be valid to mutate with `ptr`'s provenance. And"] # [doc = " there must be no Rust references referring to that memory."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/munmap.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/munmap.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/munmap.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=munmap&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/munmap.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/munmap.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=munmap&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/munmap"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Memory_002dmapped-I_002fO.html#index-munmap"] # [inline] pub unsafe fn munmap (ptr : * mut c_void , len : usize) -> io :: Result < () > { backend :: mm :: syscalls :: munmap (ptr , len) }
};
}
