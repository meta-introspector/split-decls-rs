macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! statvfs {
    () => {
        deps!();
        # [doc = " `statvfs`—Queries filesystem metadata, POSIX version."] # [doc = ""] # [doc = " Compared to [`statfs`], this function often provides less information, but"] # [doc = " it is more portable. But even so, filesystems are very diverse and not all"] # [doc = " the fields are meaningful for every filesystem. And `f_fsid` doesn't seem"] # [doc = " to have a clear meaning anywhere."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/statvfs.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/statvfs.2.html"] # [cfg (not (any (target_os = "wasi")))] # [inline] pub fn statvfs < P : path :: Arg > (path : P) -> io :: Result < StatVfs > { path . into_with_c_str (backend :: fs :: syscalls :: statvfs) }
    };
}

statvfs!();