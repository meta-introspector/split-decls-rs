macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fchdir {
    () => {
        deps!();
        # [doc = " `fchdir(fd)`—Change the current working directory."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fchdir.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fchdir.2.html"] # [cfg (not (target_os = "fuchsia"))] # [inline] pub fn fchdir < Fd : AsFd > (fd : Fd) -> io :: Result < () > { backend :: process :: syscalls :: fchdir (fd . as_fd ()) }
    };
}

fchdir!();