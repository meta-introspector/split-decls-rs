macro_rules! deps {
    () => {
        Timestamps!();
        Result!();
        Arg!();
    };
}

macro_rules! utimensat {
    () => {
        deps!();
        # [doc = " `utimensat(dirfd, path, times, flags)`—Sets file or directory timestamps."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/utimensat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/utimensat.2.html"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon" , target_os = "vita")))] # [inline] pub fn utimensat < P : path :: Arg , Fd : AsFd > (dirfd : Fd , path : P , times : & Timestamps , flags : AtFlags ,) -> io :: Result < () > { path . into_with_c_str (| path | backend :: fs :: syscalls :: utimensat (dirfd . as_fd () , path , times , flags)) }
    };
}

utimensat!()