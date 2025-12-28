macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! accept {
    () => {
        deps!();
        # [doc = " `accept(fd, NULL, NULL)`—Accepts an incoming connection."] # [doc = ""] # [doc = " Use [`acceptfrom`] to retrieve the peer address."] # [doc = ""] # [doc = " POSIX guarantees that `accept` will use the lowest unused file descriptor,"] # [doc = " however it is not safe in general to rely on this, as file descriptors may"] # [doc = " be unexpectedly allocated on other threads or in libraries."] # [doc = ""] # [doc = " See [`accept_with`] to pass additional flags."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Beej's Guide to Network Programming]"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [Winsock]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [Beej's Guide to Network Programming]: https://beej.us/guide/bgnet/html/split/system-calls-or-bust.html#acceptthank-you-for-calling-port-3490."] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/accept.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/accept.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/accept.2.html"] # [doc = " [Winsock]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=accept&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/accept.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/accept.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=accept&section=2"] # [doc = " [illumos]: https://illumos.org/man/3SOCKET/accept"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Accepting-Connections.html"] # [inline] pub fn accept < Fd : AsFd > (sockfd : Fd) -> io :: Result < OwnedFd > { backend :: net :: syscalls :: accept (sockfd . as_fd ()) }
    };
}

accept!()