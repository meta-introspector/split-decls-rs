// Generated macro for io_uring_register (function)
macro_rules! Depcrate_io_uringio_uring_register {
() => {
// Module: crate::io_uring
// Provides: {"io_uring_register"}
// Dependencies: {}
# [doc = " `io_uring_register(fd, opcode, arg, nr_args)`—Register files or user"] # [doc = " buffers for asynchronous I/O."] # [doc = ""] # [doc = " To pass flags, use [`io_uring_register_with`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " io_uring operates on raw pointers and raw file descriptors. Users are"] # [doc = " responsible for ensuring that memory and resources are only accessed in"] # [doc = " valid ways."] # [doc = ""] # [doc = " If `opcode` is `IoringRegisterOp::RegisterRingFds`, `arg` must point to"] # [doc = " mutable memory, despite being `*const`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.man7.org/linux/man-pages/man2/io_uring_register.2.html"] # [inline] pub unsafe fn io_uring_register < Fd : AsFd > (fd : Fd , opcode : IoringRegisterOp , arg : * const c_void , nr_args : u32 ,) -> io :: Result < u32 > { backend :: io_uring :: syscalls :: io_uring_register (fd . as_fd () , opcode , arg , nr_args) }
};
}
