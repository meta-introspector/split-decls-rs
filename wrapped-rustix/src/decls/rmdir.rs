macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! rmdir {
    () => {
        deps!();
        # [doc = " `rmdir(path)`—Removes a directory."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/rmdir.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/rmdir.2.html"] # [inline] pub fn rmdir < P : path :: Arg > (path : P) -> io :: Result < () > { path . into_with_c_str (backend :: fs :: syscalls :: rmdir) }
    };
}

rmdir!();