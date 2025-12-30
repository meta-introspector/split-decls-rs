// Generated macro for fcntl_getpipe_size (function)
macro_rules! Depcrate_pipefcntl_getpipe_size {
() => {
// Module: crate::pipe
// Provides: {"fcntl_getpipe_size"}
// Dependencies: {}
# [doc = " `fnctl(fd, F_GETPIPE_SZ)`—Return the buffer capacity of a pipe."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [cfg (linux_kernel)] # [inline] pub fn fcntl_getpipe_size < Fd : AsFd > (fd : Fd) -> io :: Result < usize > { backend :: pipe :: syscalls :: fcntl_getpipe_size (fd . as_fd ()) }
};
}
