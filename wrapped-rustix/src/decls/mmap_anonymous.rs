macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! mmap_anonymous {
    () => {
        deps!();
        # [doc = " `mmap(ptr, len, prot, MAP_ANONYMOUS | flags, -1, 0)`—Create an anonymous"] # [doc = " memory mapping."] # [doc = ""] # [doc = " For file-backed mappings, see [`mmap`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If `ptr` is not null, it must be aligned to the applicable page size, and"] # [doc = " the range of memory starting at `ptr` and extending for `len` bytes,"] # [doc = " rounded up to the applicable page size, must be valid to mutate with"] # [doc = " `ptr`'s provenance."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/mmap.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mmap.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mmap.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=mmap&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/mmap.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/mmap.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=mmap&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/mmap"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Memory_002dmapped-I_002fO.html#index-mmap"] # [inline] # [doc (alias = "mmap")] pub unsafe fn mmap_anonymous (ptr : * mut c_void , len : usize , prot : ProtFlags , flags : MapFlags ,) -> io :: Result < * mut c_void > { backend :: mm :: syscalls :: mmap_anonymous (ptr , len , prot , flags) }
    };
}

mmap_anonymous!();