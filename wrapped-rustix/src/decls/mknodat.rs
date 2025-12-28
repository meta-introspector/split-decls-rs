macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! mknodat {
    () => {
        deps!();
        # [doc = " `mknodat(dirfd, path, mode, dev)`—Creates special or normal files."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/mknodat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mknodat.2.html"] # [cfg (not (any (apple , target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_os = "wasi")))] # [inline] pub fn mknodat < P : path :: Arg , Fd : AsFd > (dirfd : Fd , path : P , file_type : FileType , mode : Mode , dev : Dev ,) -> io :: Result < () > { path . into_with_c_str (| path | { backend :: fs :: syscalls :: mknodat (dirfd . as_fd () , path , file_type , mode , dev) }) }
    };
}

mknodat!()