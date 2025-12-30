// Generated macro for msync (function)
macro_rules! Depcrate_mm_msyncmsync {
() => {
// Module: crate::mm::msync
// Provides: {"msync"}
// Dependencies: {}
# [doc = " `msync(addr, len, flags)`—Synchronizes a memory-mapping with its backing"] # [doc = " storage."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `addr` must be a valid pointer to memory that is appropriate to call"] # [doc = " `msync` on. Some forms of `msync` may mutate the memory or evoke a variety"] # [doc = " of side-effects on the mapping and/or the file."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/msync.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/msync.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/msync.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=msync&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/msync.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/msync.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=msync&section=2"] # [doc = " [illumos]: https://illumos.org/man/3C/msync"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Memory_002dmapped-I_002fO.html#index-msync"] # [inline] pub unsafe fn msync (addr : * mut c_void , len : usize , flags : MsyncFlags) -> io :: Result < () > { backend :: mm :: syscalls :: msync (addr , len , flags) }
};
}
