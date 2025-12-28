macro_rules! deps {
    () => {
        Bit!();
        Unsigned!();
        B0!();
        UInt!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl < U : Unsigned , B : Bit > core :: fmt :: Binary for UInt < UInt < U , B > , B0 > where UInt < U , B > : core :: fmt :: Binary , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{:b}0" , UInt ::< U , B >:: new ()) } }
    };
}

impl_353!();