macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fstat {
    () => {
        deps!();
        # [doc = " `fstat(fd)`—Queries metadata for an open file or directory."] # [doc = ""] # [doc = " [`Mode::from_raw_mode`] and [`FileType::from_raw_mode`] may be used to"] # [doc = " interpret the `st_mode` field."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fstat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fstat.2.html"] # [doc = " [`Mode::from_raw_mode`]: Mode::from_raw_mode"] # [doc = " [`FileType::from_raw_mode`]: crate::fs::FileType::from_raw_mode"] # [inline] pub fn fstat < Fd : AsFd > (fd : Fd) -> io :: Result < Stat > { backend :: fs :: syscalls :: fstat (fd . as_fd ()) }
    };
}

fstat!();