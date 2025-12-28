macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! mlock {
    () => {
        deps!();
        # [doc = " `mlock(ptr, len)`—Lock memory into RAM."] # [doc = ""] # [doc = " Some implementations implicitly round the memory region out to the nearest"] # [doc = " page boundaries, so this function may lock more memory than explicitly"] # [doc = " requested if the memory isn't page-aligned. Other implementations fail if"] # [doc = " the memory isn't page-aligned."] # [doc = ""] # [doc = " See [`mlock_with`] to pass additional flags."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The range of memory starting at `ptr`, rounded down to the applicable page"] # [doc = " boundary, and extending for `len` bytes, rounded up to the applicable page"] # [doc = " size, must be valid to read with `ptr`'s provenance."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/mlock.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mlock.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mlock.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=mlock&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/mlock.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/mlock.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=mlock&section=2"] # [doc = " [illumos]: https://illumos.org/man/3C/mlock"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Page-Lock-Functions.html#index-mlock"] # [inline] pub unsafe fn mlock (ptr : * mut c_void , len : usize) -> io :: Result < () > { backend :: mm :: syscalls :: mlock (ptr , len) }
    };
}

mlock!();