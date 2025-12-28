macro_rules! deps {
    () => {
        Result!();
        Pid!();
    };
}

macro_rules! impl_1658 {
    () => {
        deps!();
        impl fmt :: UpperHex for Pid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1658!();