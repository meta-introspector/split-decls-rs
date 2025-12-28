macro_rules! impl_396 {
    () => {
        impl Hash for io_uring_user_data { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { unsafe { self . u64_ . hash (state) } } }
    };
}

impl_396!();