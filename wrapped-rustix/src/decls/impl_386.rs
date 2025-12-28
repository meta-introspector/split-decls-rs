macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! impl_386 {
    () => {
        deps!();
        impl core :: fmt :: Pointer for io_uring_ptr { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . ptr . fmt (f) } }
    };
}

impl_386!();