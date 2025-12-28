macro_rules! deps {
    () => {
        StableCrateId!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl fmt :: LowerHex for StableCrateId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . 0 , f) } }
    };
}

impl_101!();