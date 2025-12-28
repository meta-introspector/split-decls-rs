macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! readv {
    () => {
        deps!();
        # [doc = " `readv(fd, bufs)`—Reads from a stream into multiple buffers."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/readv.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/readv.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/readv.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=readv&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/readv.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/readv.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=readv&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/readv"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Scatter_002dGather.html#index-readv"] # [cfg (not (any (windows , target_os = "espidf" , target_os = "horizon")))] # [inline] pub fn readv < Fd : AsFd > (fd : Fd , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { backend :: io :: syscalls :: readv (fd . as_fd () , bufs) }
    };
}

readv!();