macro_rules! impl_384 {
    () => {
        impl Hash for io_uring_ptr { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . ptr . hash (state) } }
    };
}

impl_384!();