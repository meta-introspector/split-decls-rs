// Generated macro for wait_bitset (function)
macro_rules! Depcrate_thread_futexwait_bitset {
() => {
// Module: crate::thread::futex
// Provides: {"wait_bitset"}
// Dependencies: {}
# [doc = " `syscall(SYS_futex, uaddr, FUTEX_WAIT_BITSET, val, timeout, NULL, val3)`"] # [doc = ""] # [doc = " This is a very low-level feature for implementing synchronization"] # [doc = " primitives. See the references links."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `futex` system call]"] # [doc = "  - [Linux `futex` feature]"] # [doc = ""] # [doc = " [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html"] # [doc = " [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html"] # [inline] pub fn wait_bitset (uaddr : & AtomicU32 , flags : Flags , val : u32 , timeout : Option < & Timespec > , val3 : NonZeroU32 ,) -> io :: Result < () > { unsafe { futex_timeout (uaddr , Operation :: WaitBitset , flags , val , timeout , ptr :: null () , val3 . get () ,) . map (| val | { debug_assert_eq ! (val , 0 , "The return value should always equal zero, if the call is successful") ; }) } }
};
}
