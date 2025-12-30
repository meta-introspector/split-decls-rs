// Generated macro for sendmsg (function)
macro_rules! Depcrate_net_send_recv_msgsendmsg {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"sendmsg"}
// Dependencies: {}
# [doc = " `sendmsg(msghdr)`—Sends a message on a socket."] # [doc = ""] # [doc = " This function is for use on connected sockets, as it doesn't have a way to"] # [doc = " specify an address. See [`sendmsg_addr`] to send messages on unconnected"] # [doc = " sockets."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/sendmsg.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/sendmsg.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sendmsg.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=sendmsg&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/sendmsg.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/sendmsg.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=sendmsg&section=2"] # [doc = " [illumos]: https://illumos.org/man/3SOCKET/sendmsg"] # [inline] pub fn sendmsg < Fd : AsFd > (socket : Fd , iov : & [IoSlice < '_ >] , control : & mut SendAncillaryBuffer < '_ , '_ , '_ > , flags : SendFlags ,) -> io :: Result < usize > { backend :: net :: syscalls :: sendmsg (socket . as_fd () , iov , control , flags) }
};
}
