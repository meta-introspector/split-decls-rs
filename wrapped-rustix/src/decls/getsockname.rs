macro_rules! deps {
    () => {
        Result!();
        SocketAddrAny!();
    };
}

macro_rules! getsockname {
    () => {
        deps!();
        # [doc = " `getsockname(fd, addr, len)`—Returns the address a socket is bound to."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [Winsock]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getsockname.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getsockname.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getsockname.2.html"] # [doc = " [Winsock]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getsockname"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=getsockname&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/getsockname.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/getsockname.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=getsockname&section=2"] # [doc = " [illumos]: https://illumos.org/man/3SOCKET/getsockname"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Reading-Address.html"] # [inline] pub fn getsockname < Fd : AsFd > (sockfd : Fd) -> io :: Result < SocketAddrAny > { backend :: net :: syscalls :: getsockname (sockfd . as_fd ()) }
    };
}

getsockname!();