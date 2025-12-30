// Generated macro for io_uring_setup (function)
macro_rules! Depcrate_io_uringio_uring_setup {
() => {
// Module: crate::io_uring
// Provides: {"io_uring_setup"}
// Dependencies: {}
# [doc = " `io_uring_setup(entries, params)`—Setup a context for performing"] # [doc = " asynchronous I/O."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If [`IoringSetupFlags::ATTACH_WQ`] is set, the `wq_fd` field of"] # [doc = " `io_uring_params` must be an open file descriptor."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.man7.org/linux/man-pages/man2/io_uring_setup.2.html"] # [inline] pub unsafe fn io_uring_setup (entries : u32 , params : & mut io_uring_params) -> io :: Result < OwnedFd > { backend :: io_uring :: syscalls :: io_uring_setup (entries , params) }
};
}
