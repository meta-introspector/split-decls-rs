macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! unlinkat {
    () => {
        deps!();
        # [doc = " `unlinkat(fd, path, flags)`—Unlinks a file or remove a directory."] # [doc = ""] # [doc = " With the [`REMOVEDIR`] flag, this removes a directory. This is in place of"] # [doc = " a `rmdirat` function."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [`REMOVEDIR`]: AtFlags::REMOVEDIR"] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/unlinkat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/unlinkat.2.html"] # [cfg (not (target_os = "espidf"))] # [inline] pub fn unlinkat < P : path :: Arg , Fd : AsFd > (dirfd : Fd , path : P , flags : AtFlags) -> io :: Result < () > { path . into_with_c_str (| path | backend :: fs :: syscalls :: unlinkat (dirfd . as_fd () , path , flags)) }
    };
}

unlinkat!();