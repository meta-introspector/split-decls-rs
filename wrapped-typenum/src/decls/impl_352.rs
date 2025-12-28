macro_rules! deps {
    () => {
        UInt!();
        B1!();
        UTerm!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl core :: fmt :: Binary for UInt < UTerm , B1 > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "1") } }
    };
}

impl_352!();