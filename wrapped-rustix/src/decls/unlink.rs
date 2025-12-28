macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! unlink {
    () => {
        deps!();
        # [doc = " `unlink(path)`—Unlinks a file."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/unlink.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/unlink.2.html"] # [inline] pub fn unlink < P : path :: Arg > (path : P) -> io :: Result < () > { path . into_with_c_str (backend :: fs :: syscalls :: unlink) }
    };
}

unlink!()