macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! io_uring_enter {
    () => {
        deps!();
        # [doc = " `io_uring_enter(fd, to_submit, min_complete, flags, 0, 0)`—Initiate"] # [doc = " and/or complete asynchronous I/O."] # [doc = ""] # [doc = " This version has no `arg` argument. To pass:"] # [doc = "  - a signal mask, use [`io_uring_enter_sigmask`]."] # [doc = "  - an [`io_uring_getevents_arg`], use [`io_uring_enter_arg`] (aka"] # [doc = "    `io_uring_enter2`)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " io_uring operates on raw pointers and raw file descriptors. Users are"] # [doc = " responsible for ensuring that memory and resources are only accessed in"] # [doc = " valid ways."] # [doc = ""] # [doc = " And, `flags` must not have [`IoringEnterFlags::EXT_ARG`] or"] # [doc = " [`IoringEnterFlags::EXT_ARG_REG`] set."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.man7.org/linux/man-pages/man2/io_uring_enter.2.html"] # [doc (alias = "io_uring_enter2")] # [inline] pub unsafe fn io_uring_enter < Fd : AsFd > (fd : Fd , to_submit : u32 , min_complete : u32 , flags : IoringEnterFlags ,) -> io :: Result < u32 > { debug_assert ! (! flags . contains (IoringEnterFlags :: EXT_ARG)) ; debug_assert ! (! flags . contains (IoringEnterFlags :: EXT_ARG_REG)) ; backend :: io_uring :: syscalls :: io_uring_enter (fd . as_fd () , to_submit , min_complete , flags , null_mut () , 0 ,) }
    };
}

io_uring_enter!()