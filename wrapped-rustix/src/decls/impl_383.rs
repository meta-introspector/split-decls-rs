macro_rules! impl_383 {
    () => {
        impl Ord for io_uring_ptr { # [inline] fn cmp (& self , other : & Self) -> Ordering { self . ptr . cmp (& other . ptr) } }
    };
}

impl_383!()