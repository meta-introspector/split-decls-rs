macro_rules! deps {
    () => {
        SocketAddrArg!();
        Result!();
    };
}

macro_rules! bind {
    () => {
        deps!();
        # [doc = " `bind(sockfd, addr)`—Binds a socket to an IP address."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Beej's Guide to Network Programming]"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [Winsock]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [Beej's Guide to Network Programming]: https://beej.us/guide/bgnet/html/split/system-calls-or-bust.html#bind"] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/bind.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/bind.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/bind.2.html"] # [doc = " [Winsock]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-bind"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=bind&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/bind.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/bind.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=bind&section=2"] # [doc = " [illumos]: https://illumos.org/man/3SOCKET/bind"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Setting-Address.html"] pub fn bind < Fd : AsFd > (sockfd : Fd , addr : & impl SocketAddrArg) -> io :: Result < () > { backend :: net :: syscalls :: bind (sockfd . as_fd () , addr) }
    };
}

bind!()