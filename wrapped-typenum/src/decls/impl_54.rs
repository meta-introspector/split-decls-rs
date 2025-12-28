macro_rules! deps {
    () => {
        Unsigned!();
        NonZero!();
        PInt!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < U : Unsigned + NonZero + core :: fmt :: Binary > core :: fmt :: Binary for PInt < U > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "+{:b}" , self . n) } }
    };
}

impl_54!();