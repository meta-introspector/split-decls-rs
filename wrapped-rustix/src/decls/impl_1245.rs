macro_rules! deps {
    () => {
        WaitPtr!();
        Result!();
    };
}

macro_rules! impl_1245 {
    () => {
        deps!();
        impl core :: fmt :: Debug for WaitPtr { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . ptr . fmt (f) } }
    };
}

impl_1245!()