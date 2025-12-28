macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! wake {
    () => {
        deps!();
        # [doc = " `syscall(SYS_futex, uaddr, FUTEX_WAKE, val, NULL, NULL, 0)`"] # [doc = ""] # [doc = " This is a very low-level feature for implementing synchronization"] # [doc = " primitives. See the references links."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `futex` system call]"] # [doc = "  - [Linux `futex` feature]"] # [doc = ""] # [doc = " [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html"] # [doc = " [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html"] # [inline] pub fn wake (uaddr : & AtomicU32 , flags : Flags , val : u32) -> io :: Result < usize > { unsafe { futex_val2 (uaddr , Operation :: Wake , flags , val , 0 , ptr :: null () , 0) } }
    };
}

wake!();