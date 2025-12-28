macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! impl_398 {
    () => {
        deps!();
        impl core :: fmt :: Debug for io_uring_user_data { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { unsafe { self . u64_ . fmt (f) } } }
    };
}

impl_398!()