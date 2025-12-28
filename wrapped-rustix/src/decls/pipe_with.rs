macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! pipe_with {
    () => {
        deps!();
        # [doc = " `pipe2(flags)`—Creates a pipe, with flags."] # [doc = ""] # [doc = " `pipe_with` is the same as [`pipe`] but adds an additional flags operand."] # [doc = ""] # [doc = " This function creates a pipe and returns two file descriptors, for the"] # [doc = " reading and writing ends of the pipe, respectively."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/pipe2.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=pipe2&sektion=2"] # [doc = " [NetBSD]: https://man.netbsd.org/pipe2.2"] # [doc = " [OpenBSD]: https://man.openbsd.org/pipe2.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=pipe2&section=2"] # [doc = " [illumos]: https://illumos.org/man/2/pipe2"] # [cfg (not (any (apple , target_os = "aix" , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "nto")))] # [inline] # [doc (alias = "pipe2")] pub fn pipe_with (flags : PipeFlags) -> io :: Result < (OwnedFd , OwnedFd) > { backend :: pipe :: syscalls :: pipe_with (flags) }
    };
}

pipe_with!()