macro_rules! deps {
    () => {
        SocketAddrAny!();
        Result!();
    };
}

macro_rules! acceptfrom_with {
    () => {
        deps!();
        # [doc = " `accept4(fd, &addr, &len, flags)`—Accepts an incoming connection and"] # [doc = " returns the peer address, with flags."] # [doc = ""] # [doc = " Use [`accept_with`] if the peer address isn't needed."] # [doc = ""] # [doc = " `acceptfrom_with` is the same as [`acceptfrom`] but adds an additional"] # [doc = " flags operand."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/accept4.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=accept4&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/accept4.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/accept4.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=accept4&section=2"] # [doc = " [illumos]: https://illumos.org/man/3SOCKET/accept4"] # [inline] # [doc (alias = "accept4")] pub fn acceptfrom_with < Fd : AsFd > (sockfd : Fd , flags : SocketFlags ,) -> io :: Result < (OwnedFd , Option < SocketAddrAny >) > { backend :: net :: syscalls :: acceptfrom_with (sockfd . as_fd () , flags) }
    };
}

acceptfrom_with!();