macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! getcwd {
    () => {
        deps!();
        # [doc = " `getcwd`—Return the current working directory."] # [doc = ""] # [doc = " If `reuse` already has available capacity, reuse it if possible."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getcwd.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/getcwd.3.html"] # [cfg (all (feature = "alloc" , feature = "fs"))] # [cfg (not (target_os = "wasi"))] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] # [inline] pub fn getcwd < B : Into < Vec < u8 > > > (reuse : B) -> io :: Result < CString > { _getcwd (reuse . into ()) }
    };
}

getcwd!()