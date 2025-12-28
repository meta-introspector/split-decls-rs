macro_rules! deps {
    () => {
        Arg!();
        Gid!();
        Uid!();
        Result!();
    };
}

macro_rules! chownat {
    () => {
        deps!();
        # [doc = " `fchownat(dirfd, path, owner, group, flags)`—Sets file or directory"] # [doc = " ownership."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fchownat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fchownat.2.html"] # [cfg (not (any (target_os = "espidf" , target_os = "wasi")))] # [inline] # [doc (alias = "fchownat")] pub fn chownat < P : path :: Arg , Fd : AsFd > (dirfd : Fd , path : P , owner : Option < Uid > , group : Option < Gid > , flags : AtFlags ,) -> io :: Result < () > { path . into_with_c_str (| path | { backend :: fs :: syscalls :: chownat (dirfd . as_fd () , path , owner , group , flags) }) }
    };
}

chownat!();