macro_rules! deps {
    () => {
        SocketAddrAny!();
        Result!();
    };
}

macro_rules! getpeername {
    () => {
        deps!();
        # [doc = " `getpeername(fd, addr, len)`—Returns the address a socket is connected"] # [doc = " to."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Beej's Guide to Network Programming]"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [Winsock]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [Beej's Guide to Network Programming]: https://beej.us/guide/bgnet/html/split/system-calls-or-bust.html#getpeernamewho-are-you"] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getpeername.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getpeername.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getpeername.2.html"] # [doc = " [Winsock]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getpeername"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/getpeername.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/getpeername.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=getpeername&section=2"] # [doc = " [illumos]: https://illumos.org/man/3SOCKET/getpeername"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Who-is-Connected.html"] # [inline] pub fn getpeername < Fd : AsFd > (sockfd : Fd) -> io :: Result < Option < SocketAddrAny > > { backend :: net :: syscalls :: getpeername (sockfd . as_fd ()) }
    };
}

getpeername!()