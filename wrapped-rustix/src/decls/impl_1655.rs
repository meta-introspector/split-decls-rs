macro_rules! deps {
    () => {
        Pid!();
        Result!();
    };
}

macro_rules! impl_1655 {
    () => {
        deps!();
        impl fmt :: Binary for Pid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1655!();