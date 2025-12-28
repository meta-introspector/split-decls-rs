macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! dup3 {
    () => {
        deps!();
        # [doc = " `dup3(fd, new, flags)`—Changes the [file description] of a file"] # [doc = " descriptor, with flags."] # [doc = ""] # [doc = " `dup3` is the same as [`dup2`] but adds an additional flags operand, and it"] # [doc = " fails in the case that `fd` and `new` have the same file descriptor value."] # [doc = " This additional difference is the reason this function isn't named"] # [doc = " `dup2_with`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = ""] # [doc = " [file description]: https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap03.html#tag_03_258"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/dup3.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=dup3&sektion=3"] # [doc = " [NetBSD]: https://man.netbsd.org/dup3.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/dup3.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=dup3&section=3"] # [cfg (not (any (target_os = "aix" , target_os = "espidf" , target_os = "horizon" , target_os = "nto" , target_os = "vita" , target_os = "wasi")))] # [inline] pub fn dup3 < Fd : AsFd > (fd : Fd , new : & mut OwnedFd , flags : DupFlags) -> io :: Result < () > { backend :: io :: syscalls :: dup3 (fd . as_fd () , new , flags) }
    };
}

dup3!()