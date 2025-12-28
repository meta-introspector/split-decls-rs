macro_rules! deps {
    () => {
        Pid!();
        Result!();
    };
}

macro_rules! impl_1657 {
    () => {
        deps!();
        impl fmt :: LowerHex for Pid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1657!();