macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! readlink {
    () => {
        deps!();
        # [doc = " `readlink(path)`—Reads the contents of a symlink."] # [doc = ""] # [doc = " If `reuse` is non-empty, reuse its buffer to store the result if possible."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/readlink.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/readlink.2.html"] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] # [inline] pub fn readlink < P : path :: Arg , B : Into < Vec < u8 > > > (path : P , reuse : B) -> io :: Result < CString > { path . into_with_c_str (| path | _readlink (path , reuse . into ())) }
    };
}

readlink!();