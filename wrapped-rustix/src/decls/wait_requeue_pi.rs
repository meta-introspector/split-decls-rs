macro_rules! deps {
    () => {
        Result!();
        Timespec!();
    };
}

macro_rules! wait_requeue_pi {
    () => {
        deps!();
        # [doc = " `syscall(SYS_futex, uaddr, FUTEX_WAIT_REQUEUE_PI, val, timeout, uaddr2, 0)`"] # [doc = ""] # [doc = " This is a very low-level feature for implementing synchronization"] # [doc = " primitives. See the references links."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `futex` system call]"] # [doc = "  - [Linux `futex` feature]"] # [doc = ""] # [doc = " [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html"] # [doc = " [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html"] # [inline] pub fn wait_requeue_pi (uaddr : & AtomicU32 , flags : Flags , val : u32 , timeout : Option < & Timespec > , uaddr2 : & AtomicU32 ,) -> io :: Result < () > { unsafe { futex_timeout (uaddr , Operation :: WaitRequeuePi , flags , val , timeout , uaddr2 , 0 ,) . map (| val | { debug_assert_eq ! (val , 0 , "The return value should always equal zero, if the call is successful") ; }) } }
    };
}

wait_requeue_pi!()