macro_rules! impl_380 {
    () => {
        impl PartialEq for io_uring_ptr { # [inline] fn eq (& self , other : & Self) -> bool { self . ptr . eq (& other . ptr) } }
    };
}

impl_380!();