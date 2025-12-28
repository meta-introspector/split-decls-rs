macro_rules! deps {
    () => {
        UTerm!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl core :: fmt :: Binary for UTerm { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "0") } }
    };
}

impl_350!()