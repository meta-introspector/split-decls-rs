macro_rules! deps {
    () => {
        Result!();
        Uid!();
    };
}

macro_rules! impl_1703 {
    () => {
        deps!();
        impl fmt :: Display for Uid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1703!()