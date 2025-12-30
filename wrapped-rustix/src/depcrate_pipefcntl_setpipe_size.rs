// Generated macro for fcntl_setpipe_size (function)
macro_rules! Depcrate_pipefcntl_setpipe_size {
() => {
// Module: crate::pipe
// Provides: {"fcntl_setpipe_size"}
// Dependencies: {}
# [doc = " `fnctl(fd, F_SETPIPE_SZ)`—Set the buffer capacity of a pipe."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [cfg (linux_kernel)] # [inline] pub fn fcntl_setpipe_size < Fd : AsFd > (fd : Fd , size : usize) -> io :: Result < usize > { backend :: pipe :: syscalls :: fcntl_setpipe_size (fd . as_fd () , size) }
};
}
