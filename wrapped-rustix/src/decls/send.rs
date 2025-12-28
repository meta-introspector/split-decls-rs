macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! send {
    () => {
        deps!();
        # [doc = " `send(fd, buf, flags)`—Writes data to a socket."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Beej's Guide to Network Programming]"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [Winsock]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [Beej's Guide to Network Programming]: https://beej.us/guide/bgnet/html/split/system-calls-or-bust.html#sendrecv"] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/send.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/send.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/send.2.html"] # [doc = " [Winsock]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=send&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/send.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/send.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=send&section=2"] # [doc = " [illumos]: https://illumos.org/man/3SOCKET/send"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Sending-Data.html"] # [inline] pub fn send < Fd : AsFd > (fd : Fd , buf : & [u8] , flags : SendFlags) -> io :: Result < usize > { backend :: net :: syscalls :: send (fd . as_fd () , buf , flags) }
    };
}

send!();