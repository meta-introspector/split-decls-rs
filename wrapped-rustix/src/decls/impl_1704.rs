macro_rules! deps {
    () => {
        Result!();
        Uid!();
    };
}

macro_rules! impl_1704 {
    () => {
        deps!();
        impl fmt :: Binary for Uid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1704!()