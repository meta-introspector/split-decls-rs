// Generated macro for requeue (function)
macro_rules! Depcrate_thread_futexrequeue {
() => {
// Module: crate::thread::futex
// Provides: {"requeue"}
// Dependencies: {}
# [doc = " `syscall(SYS_futex, uaddr, FUTEX_REQUEUE, val, val2, uaddr2, 0)`"] # [doc = ""] # [doc = " This is a very low-level feature for implementing synchronization"] # [doc = " primitives. See the references links."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `futex` system call]"] # [doc = "  - [Linux `futex` feature]"] # [doc = ""] # [doc = " [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html"] # [doc = " [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html"] # [inline] pub fn requeue (uaddr : & AtomicU32 , flags : Flags , val : u32 , val2 : u32 , uaddr2 : & AtomicU32 ,) -> io :: Result < usize > { unsafe { futex_val2 (uaddr , Operation :: Requeue , flags , val , val2 , uaddr2 , 0) } }
};
}
