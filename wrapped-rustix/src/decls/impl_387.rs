macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        impl core :: fmt :: Debug for io_uring_ptr { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . ptr . fmt (f) } }
    };
}

impl_387!();