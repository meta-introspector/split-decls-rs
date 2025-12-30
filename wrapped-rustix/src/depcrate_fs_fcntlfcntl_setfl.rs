// Generated macro for fcntl_setfl (function)
macro_rules! Depcrate_fs_fcntlfcntl_setfl {
() => {
// Module: crate::fs::fcntl
// Provides: {"fcntl_setfl"}
// Dependencies: {}
# [doc = " `fcntl(fd, F_SETFL, flags)`—Sets a file descriptor's status."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fcntl.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [inline] # [doc (alias = "F_SETFL")] pub fn fcntl_setfl < Fd : AsFd > (fd : Fd , flags : OFlags) -> io :: Result < () > { backend :: fs :: syscalls :: fcntl_setfl (fd . as_fd () , flags) }
};
}
