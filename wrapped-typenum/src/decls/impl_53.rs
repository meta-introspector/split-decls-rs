macro_rules! deps {
    () => {
        Z0!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl core :: fmt :: Binary for Z0 { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "0") } }
    };
}

impl_53!()