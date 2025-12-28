macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! linkat {
    () => {
        deps!();
        # [doc = " `linkat(old_dirfd, old_path, new_dirfd, new_path, flags)`—Creates a hard"] # [doc = " link."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/linkat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/linkat.2.html"] # [cfg (not (target_os = "espidf"))] # [inline] pub fn linkat < P : path :: Arg , Q : path :: Arg , PFd : AsFd , QFd : AsFd > (old_dirfd : PFd , old_path : P , new_dirfd : QFd , new_path : Q , flags : AtFlags ,) -> io :: Result < () > { old_path . into_with_c_str (| old_path | { new_path . into_with_c_str (| new_path | { backend :: fs :: syscalls :: linkat (old_dirfd . as_fd () , old_path , new_dirfd . as_fd () , new_path , flags ,) }) }) }
    };
}

linkat!();