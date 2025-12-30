// Generated macro for wake_bitset (function)
macro_rules! Depcrate_thread_futexwake_bitset {
() => {
// Module: crate::thread::futex
// Provides: {"wake_bitset"}
// Dependencies: {}
# [doc = " `syscall(SYS_futex, uaddr, FUTEX_WAKE_BITSET, val, NULL, NULL, val3)`"] # [doc = ""] # [doc = " This is a very low-level feature for implementing synchronization"] # [doc = " primitives. See the references links."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `futex` system call]"] # [doc = "  - [Linux `futex` feature]"] # [doc = ""] # [doc = " [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html"] # [doc = " [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html"] # [inline] pub fn wake_bitset (uaddr : & AtomicU32 , flags : Flags , val : u32 , val3 : NonZeroU32 ,) -> io :: Result < usize > { unsafe { futex_val2 (uaddr , Operation :: WakeBitset , flags , val , 0 , ptr :: null () , val3 . get () ,) } }
};
}
