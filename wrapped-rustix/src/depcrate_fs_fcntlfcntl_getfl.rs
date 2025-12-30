// Generated macro for fcntl_getfl (function)
macro_rules! Depcrate_fs_fcntlfcntl_getfl {
() => {
// Module: crate::fs::fcntl
// Provides: {"fcntl_getfl"}
// Dependencies: {}
# [doc = " `fcntl(fd, F_GETFL)`—Returns a file descriptor's access mode and status."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fcntl.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [inline] # [doc (alias = "F_GETFL")] pub fn fcntl_getfl < Fd : AsFd > (fd : Fd) -> io :: Result < OFlags > { backend :: fs :: syscalls :: fcntl_getfl (fd . as_fd ()) }
};
}
