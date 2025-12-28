macro_rules! deps {
    () => {
        KernelSigSet!();
        Result!();
    };
}

macro_rules! io_uring_enter_sigmask {
    () => {
        deps!();
        # [doc = " `io_uring_enter(fd, to_submit, min_complete, flags, sigmask,"] # [doc = " sizeof(*sigmask))`— Initiate and/or complete asynchronous I/O, with a"] # [doc = " signal mask."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " io_uring operates on raw pointers and raw file descriptors. Users are"] # [doc = " responsible for ensuring that memory and resources are only accessed in"] # [doc = " valid ways."] # [doc = ""] # [doc = " And, `flags` must not have [`IoringEnterFlags::EXT_ARG`] or"] # [doc = " [`IoringEnterFlags::EXT_ARG_REG`] set."] # [doc = ""] # [doc = " And, the `KernelSigSet` referred to by `arg` must not contain any signal"] # [doc = " numbers reserved by libc."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.man7.org/linux/man-pages/man2/io_uring_enter.2.html"] # [doc (alias = "io_uring_enter")] # [inline] pub unsafe fn io_uring_enter_sigmask < Fd : AsFd > (fd : Fd , to_submit : u32 , min_complete : u32 , flags : IoringEnterFlags , sigmask : Option < & KernelSigSet > ,) -> io :: Result < u32 > { debug_assert ! (! flags . contains (IoringEnterFlags :: EXT_ARG)) ; debug_assert ! (! flags . contains (IoringEnterFlags :: EXT_ARG_REG)) ; backend :: io_uring :: syscalls :: io_uring_enter (fd . as_fd () , to_submit , min_complete , flags , option_as_ptr (sigmask) . cast :: < c_void > () , size_of :: < KernelSigSet > () ,) }
    };
}

io_uring_enter_sigmask!();