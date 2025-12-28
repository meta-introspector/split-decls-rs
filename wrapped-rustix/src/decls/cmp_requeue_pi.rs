macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! cmp_requeue_pi {
    () => {
        deps!();
        # [doc = " `syscall(SYS_futex, uaddr, FUTEX_CMP_REQUEUE_PI, 1, val2, uaddr2, val3)`"] # [doc = ""] # [doc = " This is a very low-level feature for implementing synchronization"] # [doc = " primitives. See the references links."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `futex` system call]"] # [doc = "  - [Linux `futex` feature]"] # [doc = ""] # [doc = " [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html"] # [doc = " [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html"] # [inline] pub fn cmp_requeue_pi (uaddr : & AtomicU32 , flags : Flags , val2 : u32 , uaddr2 : & AtomicU32 , val3 : u32 ,) -> io :: Result < usize > { unsafe { futex_val2 (uaddr , Operation :: CmpRequeuePi , flags , 1 , val2 , uaddr2 , val3) } }
    };
}

cmp_requeue_pi!()