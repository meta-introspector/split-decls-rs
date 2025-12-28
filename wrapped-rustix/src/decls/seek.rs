macro_rules! deps {
    () => {
        Result!();
        SeekFrom!();
    };
}

macro_rules! seek {
    () => {
        deps!();
        # [doc = " `lseek(fd, offset, whence)`—Repositions a file descriptor within a file."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/lseek.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/lseek.2.html"] # [inline] # [doc (alias = "lseek")] pub fn seek < Fd : AsFd > (fd : Fd , pos : SeekFrom) -> io :: Result < u64 > { backend :: fs :: syscalls :: seek (fd . as_fd () , pos) }
    };
}

seek!();