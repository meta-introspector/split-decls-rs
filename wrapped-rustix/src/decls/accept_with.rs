macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! accept_with {
    () => {
        deps!();
        # [doc = " `accept4(fd, NULL, NULL, flags)`—Accepts an incoming connection, with"] # [doc = " flags."] # [doc = ""] # [doc = " Use [`acceptfrom_with`] to retrieve the peer address."] # [doc = ""] # [doc = " Even though POSIX guarantees that this will use the lowest unused file"] # [doc = " descriptor, it is not safe in general to rely on this, as file descriptors"] # [doc = " may be unexpectedly allocated on other threads or in libraries."] # [doc = ""] # [doc = " `accept_with` is the same as [`accept`] but adds an additional flags"] # [doc = " operand."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/accept4.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=accept4&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/accept4.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/accept4.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=accept4&section=2"] # [doc = " [illumos]: https://illumos.org/man/3SOCKET/accept4"] # [inline] # [doc (alias = "accept4")] pub fn accept_with < Fd : AsFd > (sockfd : Fd , flags : SocketFlags) -> io :: Result < OwnedFd > { backend :: net :: syscalls :: accept_with (sockfd . as_fd () , flags) }
    };
}

accept_with!()