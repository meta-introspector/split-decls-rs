// Generated macro for socketpair (function)
macro_rules! Depcrate_net_socketpairsocketpair {
() => {
// Module: crate::net::socketpair
// Provides: {"socketpair"}
// Dependencies: {}
# [doc = " `socketpair(domain, type_ | accept_flags, protocol)`—Create a pair of"] # [doc = " sockets that are connected to each other."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/socketpair.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/socketpair.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/socketpair.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=socketpair&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/socketpair.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/socketpair.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=socketpair&section=2"] # [doc = " [illumos]: https://illumos.org/man/3SOCKET/socketpair"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Socket-Pairs.html"] # [inline] pub fn socketpair (domain : AddressFamily , type_ : SocketType , flags : SocketFlags , protocol : Option < Protocol > ,) -> io :: Result < (OwnedFd , OwnedFd) > { backend :: net :: syscalls :: socketpair (domain , type_ , flags , protocol) }
};
}
