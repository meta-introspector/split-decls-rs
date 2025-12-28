macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! trylock_pi {
    () => {
        deps!();
        # [doc = " `syscall(SYS_futex, uaddr, FUTEX_TRYLOCK_PI, 0, NULL, NULL, 0)`"] # [doc = ""] # [doc = " This is a very low-level feature for implementing synchronization"] # [doc = " primitives. See the references links."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `futex` system call]"] # [doc = "  - [Linux `futex` feature]"] # [doc = ""] # [doc = " [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html"] # [doc = " [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html"] # [inline] pub fn trylock_pi (uaddr : & AtomicU32 , flags : Flags) -> io :: Result < bool > { unsafe { futex_val2 (uaddr , Operation :: TrylockPi , flags , 0 , 0 , ptr :: null () , 0) . map (| ret | ret == 0) } }
    };
}

trylock_pi!()