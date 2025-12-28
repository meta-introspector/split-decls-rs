macro_rules! deps {
    () => {
        SocketAddrAny!();
        Result!();
        Buffer!();
    };
}

macro_rules! recvfrom {
    () => {
        deps!();
        # [doc = " `recvfrom(fd, buf, flags, addr, len)`—Reads data from a socket and"] # [doc = " returns the sender address."] # [doc = ""] # [doc = " In addition to the `Buffer::Output` return value, this also returns the"] # [doc = " number of bytes received before any truncation due to the"] # [doc = " [`RecvFlags::TRUNC`] flag."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Beej's Guide to Network Programming]"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [Winsock]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [Beej's Guide to Network Programming]: https://beej.us/guide/bgnet/html/split/system-calls-or-bust.html#sendtorecv"] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/recvfrom.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/recvfrom.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/recvfrom.2.html"] # [doc = " [Winsock]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-recvfrom"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=recvfrom&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/recvfrom.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/recvfrom.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=recvfrom&section=2"] # [doc = " [illumos]: https://illumos.org/man/3SOCKET/recvfrom"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Receiving-Datagrams.html"] # [inline] pub fn recvfrom < Fd : AsFd , Buf : Buffer < u8 > > (fd : Fd , mut buf : Buf , flags : RecvFlags ,) -> io :: Result < (Buf :: Output , usize , Option < SocketAddrAny >) > { let (ptr , len) = buf . parts_mut () ; let (recv_len , addr) = unsafe { backend :: net :: syscalls :: recvfrom (fd . as_fd () , (ptr , len) , flags) ? } ; let min_len = min (len , recv_len) ; unsafe { Ok ((buf . assume_init (min_len) , recv_len , addr)) } }
    };
}

recvfrom!();