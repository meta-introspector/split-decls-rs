macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! chmod {
    () => {
        deps!();
        # [doc = " `chmod(path, mode)`—Sets file or directory permissions."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/chmod.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/chmod.2.html"] # [cfg (not (target_os = "wasi"))] # [inline] pub fn chmod < P : path :: Arg > (path : P , mode : Mode) -> io :: Result < () > { path . into_with_c_str (| path | backend :: fs :: syscalls :: chmod (path , mode)) }
    };
}

chmod!()