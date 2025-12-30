// Generated macro for fd (function)
macro_rules! Depcrate_thread_futexfd {
() => {
// Module: crate::thread::futex
// Provides: {"fd"}
// Dependencies: {}
# [doc = " `syscall(SYS_futex, uaddr, FUTEX_FD, val, NULL, NULL, 0)`"] # [doc = ""] # [doc = " This is a very low-level feature for implementing synchronization"] # [doc = " primitives. See the references links."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `futex` system call]"] # [doc = "  - [Linux `futex` feature]"] # [doc = ""] # [doc = " [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html"] # [doc = " [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html"] # [inline] pub fn fd (uaddr : & AtomicU32 , flags : Flags , val : u32) -> io :: Result < OwnedFd > { unsafe { futex_val2 (uaddr , Operation :: Fd , flags , val , 0 , ptr :: null () , 0) . map (| val | { let fd = val as RawFd ; debug_assert_eq ! (fd as usize , val , "return value should be a valid fd") ; OwnedFd :: from_raw_fd (fd) }) } }
};
}
