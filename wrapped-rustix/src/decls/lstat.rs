macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! lstat {
    () => {
        deps!();
        # [doc = " `lstat(path)`—Queries metadata for a file or directory, without following"] # [doc = " symlinks."] # [doc = ""] # [doc = " [`Mode::from_raw_mode`] and [`FileType::from_raw_mode`] may be used to"] # [doc = " interpret the `st_mode` field."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/lstat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/lstat.2.html"] # [doc = " [`Mode::from_raw_mode`]: crate::fs::Mode::from_raw_mode"] # [doc = " [`FileType::from_raw_mode`]: crate::fs::FileType::from_raw_mode"] # [inline] pub fn lstat < P : path :: Arg > (path : P) -> io :: Result < Stat > { path . into_with_c_str (backend :: fs :: syscalls :: lstat) }
    };
}

lstat!();