macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! pwritev {
    () => {
        deps!();
        # [doc = " `pwritev(fd, bufs, offset)`—Writes to a file at a given position from"] # [doc = " multiple buffers."] # [doc = ""] # [doc = " Contrary to POSIX, on many popular platforms including Linux and FreeBSD,"] # [doc = " if the file is opened in append mode, this ignores the offset appends the"] # [doc = " data to the end of the file."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/pwritev.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=pwritev&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/pwritev.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/pwritev.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=pwritev&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/pwritev"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/I_002fO-Primitives.html#index-pwrite64"] # [cfg (not (any (windows , target_os = "cygwin" , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "nto" , target_os = "redox" , target_os = "solaris" , target_os = "vita" ,)))] # [inline] pub fn pwritev < Fd : AsFd > (fd : Fd , bufs : & [IoSlice < '_ >] , offset : u64) -> io :: Result < usize > { backend :: io :: syscalls :: pwritev (fd . as_fd () , bufs , offset) }
    };
}

pwritev!();