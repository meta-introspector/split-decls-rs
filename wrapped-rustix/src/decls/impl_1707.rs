macro_rules! deps {
    () => {
        Result!();
        Uid!();
    };
}

macro_rules! impl_1707 {
    () => {
        deps!();
        impl fmt :: UpperHex for Uid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1707!()