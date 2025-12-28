macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fcntl_dupfd_cloexec {
    () => {
        deps!();
        # [doc = " `fcntl(fd, F_DUPFD_CLOEXEC)`—Creates a new `OwnedFd` instance, with value"] # [doc = " at least `min`, that has `O_CLOEXEC` set and that shares the same"] # [doc = " underlying [file description] as `fd`."] # [doc = ""] # [doc = " POSIX guarantees that `F_DUPFD_CLOEXEC` will use the lowest unused file"] # [doc = " descriptor which is at least `min`, however it is not safe in general to"] # [doc = " rely on this, as file descriptors may be unexpectedly allocated on other"] # [doc = " threads or in libraries."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fcntl.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=fcntl&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/fcntl.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/fcntl.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=fcntl&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/fcntl"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Control-Operations.html#index-fcntl-function"] # [doc = " [file description]: https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap03.html#tag_03_258"] # [cfg (not (any (target_os = "espidf" , target_os = "wasi")))] # [inline] # [doc (alias = "F_DUPFD_CLOEXEC")] pub fn fcntl_dupfd_cloexec < Fd : AsFd > (fd : Fd , min : RawFd) -> io :: Result < OwnedFd > { backend :: io :: syscalls :: fcntl_dupfd_cloexec (fd . as_fd () , min) }
    };
}

fcntl_dupfd_cloexec!();