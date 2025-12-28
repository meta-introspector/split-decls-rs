macro_rules! deps {
    () => {
        UInt!();
        Unsigned!();
        B1!();
        Bit!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        impl < U : Unsigned , B : Bit > core :: fmt :: Binary for UInt < UInt < U , B > , B1 > where UInt < U , B > : core :: fmt :: Binary , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{:b}1" , UInt ::< U , B >:: new ()) } }
    };
}

impl_354!()