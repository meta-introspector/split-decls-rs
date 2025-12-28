macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! mprotect {
    () => {
        deps!();
        # [doc = " `mprotect(ptr, len, flags)`—Change the protection flags of a region of"] # [doc = " memory."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The range of memory starting at `ptr` and extending for `len` bytes,"] # [doc = " rounded up to the applicable page size, must be valid to read with `ptr`'s"] # [doc = " provenance."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/mprotect.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mprotect.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mprotect.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=mprotect&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/mprotect.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/mprotect.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=mprotect&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/mprotect"] # [inline] pub unsafe fn mprotect (ptr : * mut c_void , len : usize , flags : MprotectFlags) -> io :: Result < () > { backend :: mm :: syscalls :: mprotect (ptr , len , flags) }
    };
}

mprotect!();