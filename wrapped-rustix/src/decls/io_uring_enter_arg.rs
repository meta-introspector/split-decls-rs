macro_rules! deps {
    () => {
        KernelSigSet!();
        Result!();
    };
}

macro_rules! io_uring_enter_arg {
    () => {
        deps!();
        # [doc = " `io_uring_enter2(fd, to_submit, min_complete, flags, arg, sizeof(*arg))`—"] # [doc = " Initiate and/or complete asynchronous I/O, with a signal mask and a"] # [doc = " timeout."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " io_uring operates on raw pointers and raw file descriptors. Users are"] # [doc = " responsible for ensuring that memory and resources are only accessed in"] # [doc = " valid ways."] # [doc = ""] # [doc = " And, `flags` must have [`IoringEnterFlags::EXT_ARG`] set, and must not have"] # [doc = " [`IoringEnterFlags::EXT_ARG_REG`] set."] # [doc = ""] # [doc = " And, the `KernelSigSet` pointed to by the `io_uring_getenvets_arg` referred"] # [doc = " to by `arg` must not contain any signal numbers reserved by libc."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.man7.org/linux/man-pages/man2/io_uring_enter.2.html"] # [doc (alias = "io_uring_enter")] # [doc (alias = "io_uring_enter2")] # [inline] pub unsafe fn io_uring_enter_arg < Fd : AsFd > (fd : Fd , to_submit : u32 , min_complete : u32 , flags : IoringEnterFlags , arg : Option < & io_uring_getevents_arg > ,) -> io :: Result < u32 > { debug_assert ! (flags . contains (IoringEnterFlags :: EXT_ARG)) ; debug_assert ! (! flags . contains (IoringEnterFlags :: EXT_ARG_REG)) ; backend :: io_uring :: syscalls :: io_uring_enter (fd . as_fd () , to_submit , min_complete , flags , option_as_ptr (arg) . cast :: < c_void > () , size_of :: < io_uring_getevents_arg > () ,) }
    };
}

io_uring_enter_arg!();