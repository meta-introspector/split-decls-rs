macro_rules! deps {
    () => {
        Buffer!();
        Result!();
    };
}

macro_rules! pread {
    () => {
        deps!();
        # [doc = " `pread(fd, buf, offset)`—Reads from a file at a given position."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/pread.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/pread.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pread.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=pread&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/pread.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/pread.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=pread&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/pread"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/I_002fO-Primitives.html#index-pread64"] # [cfg (not (windows))] # [inline] pub fn pread < Fd : AsFd , Buf : Buffer < u8 > > (fd : Fd , mut buf : Buf , offset : u64 ,) -> io :: Result < Buf :: Output > { let len = unsafe { backend :: io :: syscalls :: pread (fd . as_fd () , buf . parts_mut () , offset) ? } ; unsafe { Ok (buf . assume_init (len)) } }
    };
}

pread!()