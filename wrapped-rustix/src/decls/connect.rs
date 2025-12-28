macro_rules! deps {
    () => {
        SocketAddrArg!();
        Result!();
    };
}

macro_rules! connect {
    () => {
        deps!();
        # [doc = " `connect(sockfd, addr)`—Initiates a connection to an IP address."] # [doc = ""] # [doc = " On Windows, a non-blocking socket returns [`Errno::WOULDBLOCK`] if the"] # [doc = " connection cannot be completed immediately, rather than"] # [doc = " `Errno::INPROGRESS`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Beej's Guide to Network Programming]"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [Winsock]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [Beej's Guide to Network Programming]: https://beej.us/guide/bgnet/html/split/system-calls-or-bust.html#connect"] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/connect.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/connect.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/connect.2.html"] # [doc = " [Winsock]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=connect&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/connect.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/connect.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=connect&section=2"] # [doc = " [illumos]: https://illumos.org/man/3SOCKET/connect"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Connecting.html"] # [doc = " [`Errno::WOULDBLOCK`]: io::Errno::WOULDBLOCK"] pub fn connect < Fd : AsFd > (sockfd : Fd , addr : & impl SocketAddrArg) -> io :: Result < () > { backend :: net :: syscalls :: connect (sockfd . as_fd () , addr) }
    };
}

connect!();