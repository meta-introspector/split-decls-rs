macro_rules! deps {
    () => {
        WakeOp!();
        WakeOpCmp!();
        Result!();
    };
}

macro_rules! wake_op {
    () => {
        deps!();
        # [doc = " `syscall(SYS_futex, uaddr, FUTEX_WAKE_OP, val, val2, uaddr2, val3)`"] # [doc = ""] # [doc = " This is a very low-level feature for implementing synchronization"] # [doc = " primitives. See the references links."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `futex` system call]"] # [doc = "  - [Linux `futex` feature]"] # [doc = ""] # [doc = " [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html"] # [doc = " [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html"] # [inline] # [allow (clippy :: too_many_arguments)] pub fn wake_op (uaddr : & AtomicU32 , flags : Flags , val : u32 , val2 : u32 , uaddr2 : & AtomicU32 , op : WakeOp , cmp : WakeOpCmp , oparg : u16 , cmparg : u16 ,) -> io :: Result < usize > { if oparg >= 1 << 12 || cmparg >= 1 << 12 { return Err (io :: Errno :: INVAL) ; } let val3 = ((op as u32) << 28) | ((cmp as u32) << 24) | ((oparg as u32) << 12) | (cmparg as u32) ; unsafe { futex_val2 (uaddr , Operation :: WakeOp , flags , val , val2 , uaddr2 , val3) } }
    };
}

wake_op!()