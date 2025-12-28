macro_rules! deps {
    () => {
        NonZero!();
        Unsigned!();
        NInt!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < U : Unsigned + NonZero + core :: fmt :: Binary > core :: fmt :: Binary for NInt < U > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "-{:b}" , self . n) } }
    };
}

impl_55!();