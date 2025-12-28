macro_rules! deps {
    () => {
        B0!();
        UTerm!();
        UInt!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl core :: fmt :: Binary for UInt < UTerm , B0 > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "0") } }
    };
}

impl_351!();