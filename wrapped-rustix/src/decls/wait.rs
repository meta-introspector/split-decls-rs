macro_rules! deps {
    () => {
        Result!();
        Timespec!();
        Wait!();
    };
}

macro_rules! wait {
    () => {
        deps!();
        # [doc = " `syscall(SYS_futex, uaddr, FUTEX_WAIT, val, timeout, NULL, 0)`"] # [doc = ""] # [doc = " This is a very low-level feature for implementing synchronization"] # [doc = " primitives. See the references links."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `futex` system call]"] # [doc = "  - [Linux `futex` feature]"] # [doc = ""] # [doc = " [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html"] # [doc = " [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html"] # [inline] pub fn wait (uaddr : & AtomicU32 , flags : Flags , val : u32 , timeout : Option < & Timespec > ,) -> io :: Result < () > { unsafe { futex_timeout (uaddr , Operation :: Wait , flags , val , timeout , ptr :: null () , 0) . map (| val | { debug_assert_eq ! (val , 0 , "The return value should always equal zero, if the call is successful") ; }) } }
    };
}

wait!()