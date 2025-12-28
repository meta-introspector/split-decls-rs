macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ttyname {
    () => {
        deps!();
        # [doc = " `ttyname_r(fd)`—Returns the name of the tty open on `fd`."] # [doc = ""] # [doc = " If `reuse` already has available capacity, reuse it if possible."] # [doc = ""] # [doc = " On Linux, this function depends on procfs being mounted on /proc."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/ttyname.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/ttyname.3.html"] # [cfg (not (any (target_os = "fuchsia" , target_os = "wasi")))] # [cfg (feature = "alloc")] # [cfg (feature = "fs")] # [doc (alias = "ttyname_r")] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] # [inline] pub fn ttyname < Fd : AsFd , B : Into < Vec < u8 > > > (fd : Fd , reuse : B) -> io :: Result < CString > { _ttyname (fd . as_fd () , reuse . into ()) }
    };
}

ttyname!()