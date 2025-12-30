// Generated macro for mlockall (function)
macro_rules! Depcrate_mm_mmapmlockall {
() => {
// Module: crate::mm::mmap
// Provides: {"mlockall"}
// Dependencies: {}
# [doc = " Locks all pages mapped into the address space of the calling process."] # [doc = ""] # [doc = " This includes the pages of the code, data, and stack segment, as well as"] # [doc = " shared libraries, user space kernel data, shared memory, and memory-mapped"] # [doc = " files. All mapped pages are guaranteed to be resident in RAM when the call"] # [doc = " returns successfully; the pages are guaranteed to stay in RAM until later"] # [doc = " unlocked."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/mlockall.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mlockall.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=mlockall&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/mlockall.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/mlockall.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=mlockall&section=2"] # [doc = " [illumos]: https://illumos.org/man/3C/mlockall"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Page-Lock-Functions.html#index-mlockall"] # [cfg (any (linux_kernel , freebsdlike , netbsdlike))] # [inline] pub fn mlockall (flags : MlockAllFlags) -> io :: Result < () > { backend :: mm :: syscalls :: mlockall (flags) }
};
}
