macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fchmod {
    () => {
        deps!();
        # [doc = " `fchmod(fd, mode)`—Sets open file or directory permissions."] # [doc = ""] # [doc = " This implementation does not support [`OFlags::PATH`] file descriptors,"] # [doc = " even on platforms where the host libc emulates it."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fchmod.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fchmod.2.html"] # [doc = " [`OFlags::PATH`]: crate::fs::OFlags::PATH"] # [cfg (not (target_os = "wasi"))] # [inline] pub fn fchmod < Fd : AsFd > (fd : Fd , mode : Mode) -> io :: Result < () > { backend :: fs :: syscalls :: fchmod (fd . as_fd () , mode) }
    };
}

fchmod!()