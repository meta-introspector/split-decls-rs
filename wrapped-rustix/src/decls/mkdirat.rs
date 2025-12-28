macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! mkdirat {
    () => {
        deps!();
        # [doc = " `mkdirat(fd, path, mode)`—Creates a directory."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/mkdirat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mkdirat.2.html"] # [inline] pub fn mkdirat < P : path :: Arg , Fd : AsFd > (dirfd : Fd , path : P , mode : Mode) -> io :: Result < () > { path . into_with_c_str (| path | backend :: fs :: syscalls :: mkdirat (dirfd . as_fd () , path , mode)) }
    };
}

mkdirat!();