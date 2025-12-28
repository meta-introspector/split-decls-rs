macro_rules! deps {
    () => {
        Uid!();
        Result!();
    };
}

macro_rules! impl_1709 {
    () => {
        deps!();
        impl fmt :: UpperExp for Uid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1709!();