macro_rules! impl_395 {
    () => {
        impl Ord for io_uring_user_data { # [inline] fn cmp (& self , other : & Self) -> Ordering { unsafe { self . u64_ . cmp (& other . u64_) } } }
    };
}

impl_395!()