macro_rules! deps {
    () => {
        RecvAncillaryBuffer!();
        Result!();
        RecvMsg!();
    };
}

macro_rules! recvmsg {
    () => {
        deps!();
        # [doc = " `recvmsg(msghdr)`—Receives a message from a socket."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/recvmsg.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/recvmsg.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/recvmsg.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=recvmsg&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/recvmsg.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/recvmsg.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=recvmsg&section=2"] # [doc = " [illumos]: https://illumos.org/man/3SOCKET/recvmsg"] # [inline] pub fn recvmsg < Fd : AsFd > (socket : Fd , iov : & mut [IoSliceMut < '_ >] , control : & mut RecvAncillaryBuffer < '_ > , flags : RecvFlags ,) -> io :: Result < RecvMsg > { backend :: net :: syscalls :: recvmsg (socket . as_fd () , iov , control , flags) }
    };
}

recvmsg!();