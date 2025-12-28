macro_rules! deps {
    () => {
        Uid!();
        Result!();
    };
}

macro_rules! impl_1705 {
    () => {
        deps!();
        impl fmt :: Octal for Uid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1705!();