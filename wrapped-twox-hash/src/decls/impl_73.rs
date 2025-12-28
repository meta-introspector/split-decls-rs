macro_rules! deps {
    () => {
        OneshotWithSecretError!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl core :: fmt :: Display for OneshotWithSecretError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_73!();