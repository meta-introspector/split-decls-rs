// Generated macro for unlock_pi (function)
macro_rules! Depcrate_thread_futexunlock_pi {
() => {
// Module: crate::thread::futex
// Provides: {"unlock_pi"}
// Dependencies: {}
# [doc = " `syscall(SYS_futex, uaddr, FUTEX_UNLOCK_PI, 0, NULL, NULL, 0)`"] # [doc = ""] # [doc = " This is a very low-level feature for implementing synchronization"] # [doc = " primitives. See the references links."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `futex` system call]"] # [doc = "  - [Linux `futex` feature]"] # [doc = ""] # [doc = " [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html"] # [doc = " [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html"] # [inline] pub fn unlock_pi (uaddr : & AtomicU32 , flags : Flags) -> io :: Result < () > { unsafe { futex_val2 (uaddr , Operation :: UnlockPi , flags , 0 , 0 , ptr :: null () , 0) . map (| val | { debug_assert_eq ! (val , 0 , "The return value should always equal zero, if the call is successful") ; }) } }
};
}
