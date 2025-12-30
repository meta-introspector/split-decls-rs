// Generated macro for munlockall (function)
macro_rules! Depcrate_mm_mmapmunlockall {
() => {
// Module: crate::mm::mmap
// Provides: {"munlockall"}
// Dependencies: {}
# [doc = " Unlocks all pages mapped into the address space of the calling process."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This function is aware of all the memory pages in the process, as if it"] # [doc = " were a debugger. It unlocks all the pages, which could potentially"] # [doc = " compromise security assumptions made by code about memory it has"] # [doc = " encapsulated."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/munlockall.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/munlockall.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=munlockall&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/munlockall.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/munlockall.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=munlockall&section=2"] # [doc = " [illumos]: https://illumos.org/man/3C/munlockall"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Page-Lock-Functions.html#index-munlockall"] # [cfg (any (linux_kernel , freebsdlike , netbsdlike))] # [inline] pub fn munlockall () -> io :: Result < () > { backend :: mm :: syscalls :: munlockall () }
};
}
