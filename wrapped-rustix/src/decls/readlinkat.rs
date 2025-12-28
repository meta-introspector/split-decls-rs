macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! readlinkat {
    () => {
        deps!();
        # [doc = " `readlinkat(fd, path)`—Reads the contents of a symlink."] # [doc = ""] # [doc = " If `reuse` already has available capacity, reuse it if possible."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/readlinkat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/readlinkat.2.html"] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] # [inline] pub fn readlinkat < P : path :: Arg , Fd : AsFd , B : Into < Vec < u8 > > > (dirfd : Fd , path : P , reuse : B ,) -> io :: Result < CString > { path . into_with_c_str (| path | _readlinkat (dirfd . as_fd () , path , reuse . into ())) }
    };
}

readlinkat!()