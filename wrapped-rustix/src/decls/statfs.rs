macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! statfs {
    () => {
        deps!();
        # [doc = " `statfs`—Queries filesystem metadata."] # [doc = ""] # [doc = " Compared to [`statvfs`], this function often provides more information,"] # [doc = " though it's less portable."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/statfs.2.html"] # [cfg (not (any (solarish , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "netbsd" , target_os = "nto" , target_os = "redox" , target_os = "vita" , target_os = "wasi" ,)))] # [inline] pub fn statfs < P : path :: Arg > (path : P) -> io :: Result < StatFs > { path . into_with_c_str (backend :: fs :: syscalls :: statfs) }
    };
}

statfs!()