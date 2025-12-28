macro_rules! deps {
    () => {
        IoringRegisterOp!();
        Result!();
    };
}

macro_rules! io_uring_register_with {
    () => {
        deps!();
        # [doc = " `io_uring_register_with(fd, opcode, flags, arg, nr_args)`—Register files or"] # [doc = " user buffers for asynchronous I/O."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " io_uring operates on raw pointers and raw file descriptors. Users are"] # [doc = " responsible for ensuring that memory and resources are only accessed in"] # [doc = " valid ways."] # [doc = ""] # [doc = " If `opcode` is `IoringRegisterOp::RegisterRingFds`, `arg` must point to"] # [doc = " mutable memory, despite being `*const`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.man7.org/linux/man-pages/man2/io_uring_register.2.html"] # [inline] pub unsafe fn io_uring_register_with < Fd : AsFd > (fd : Fd , opcode : IoringRegisterOp , flags : IoringRegisterFlags , arg : * const c_void , nr_args : u32 ,) -> io :: Result < u32 > { backend :: io_uring :: syscalls :: io_uring_register_with (fd . as_fd () , opcode , flags , arg , nr_args) }
    };
}

io_uring_register_with!()