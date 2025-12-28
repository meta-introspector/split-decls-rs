macro_rules! deps {
    () => {
        Result!();
        Shutdown!();
    };
}

macro_rules! shutdown {
    () => {
        deps!();
        # [doc = " `shutdown(fd, how)`—Closes the read and/or write sides of a stream."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Beej's Guide to Network Programming]"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [Winsock]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [Beej's Guide to Network Programming]: https://beej.us/guide/bgnet/html/split/system-calls-or-bust.html#close-and-shutdownget-outta-my-face"] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/shutdown.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/shutdown.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/shutdown.2.html"] # [doc = " [Winsock]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-shutdown"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=shutdown&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/shutdown.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/shutdown.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=shutdown&section=2"] # [doc = " [illumos]: https://illumos.org/man/3SOCKET/shutdown"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Closing-a-Socket.html"] # [inline] pub fn shutdown < Fd : AsFd > (sockfd : Fd , how : Shutdown) -> io :: Result < () > { backend :: net :: syscalls :: shutdown (sockfd . as_fd () , how) }
    };
}

shutdown!()