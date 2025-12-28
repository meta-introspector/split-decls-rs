macro_rules! deps {
    () => {
        Buffer!();
        Result!();
    };
}

macro_rules! read {
    () => {
        deps!();
        # [doc = " `read(fd, buf)`—Reads from a stream."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/read.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/read.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/read.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=read&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/read.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/read.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=read&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/read"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/I_002fO-Primitives.html#index-reading-from-a-file-descriptor"] # [inline] pub fn read < Fd : AsFd , Buf : Buffer < u8 > > (fd : Fd , mut buf : Buf) -> io :: Result < Buf :: Output > { let len = unsafe { backend :: io :: syscalls :: read (fd . as_fd () , buf . parts_mut ()) ? } ; unsafe { Ok (buf . assume_init (len)) } }
    };
}

read!()