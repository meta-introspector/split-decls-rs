macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! io_uring_setup {
    () => {
        deps!();
        # [doc = " `io_uring_setup(entries, params)`—Setup a context for performing"] # [doc = " asynchronous I/O."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If [`IoringSetupFlags::ATTACH_WQ`] is set, the `wq_fd` field of"] # [doc = " `io_uring_params` must be an open file descriptor."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.man7.org/linux/man-pages/man2/io_uring_setup.2.html"] # [inline] pub unsafe fn io_uring_setup (entries : u32 , params : & mut io_uring_params) -> io :: Result < OwnedFd > { backend :: io_uring :: syscalls :: io_uring_setup (entries , params) }
    };
}

io_uring_setup!()