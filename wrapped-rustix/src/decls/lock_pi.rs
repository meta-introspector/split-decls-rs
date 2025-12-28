macro_rules! deps {
    () => {
        Timespec!();
        Result!();
    };
}

macro_rules! lock_pi {
    () => {
        deps!();
        # [doc = " `syscall(SYS_futex, uaddr, FUTEX_LOCK_PI, 0, timeout, NULL, 0)`"] # [doc = ""] # [doc = " This is a very low-level feature for implementing synchronization"] # [doc = " primitives. See the references links."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `futex` system call]"] # [doc = "  - [Linux `futex` feature]"] # [doc = ""] # [doc = " [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html"] # [doc = " [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html"] # [inline] pub fn lock_pi (uaddr : & AtomicU32 , flags : Flags , timeout : Option < & Timespec >) -> io :: Result < () > { unsafe { futex_timeout (uaddr , Operation :: LockPi , flags , 0 , timeout , ptr :: null () , 0) . map (| val | { debug_assert_eq ! (val , 0 , "The return value should always equal zero, if the call is successful") ; }) } }
    };
}

lock_pi!()