macro_rules! deps {
    () => {
        Result!();
        Timestamps!();
    };
}

macro_rules! futimens {
    () => {
        deps!();
        # [doc = " `futimens(fd, times)`—Sets timestamps for an open file or directory."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/futimens.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/utimensat.2.html"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon" , target_os = "vita")))] # [inline] pub fn futimens < Fd : AsFd > (fd : Fd , times : & Timestamps) -> io :: Result < () > { backend :: fs :: syscalls :: futimens (fd . as_fd () , times) }
    };
}

futimens!()