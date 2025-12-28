macro_rules! deps {
    () => {
        ToInt!();
        Bit!();
        Unsigned!();
        UInt!();
    };
}

macro_rules! impl_523 {
    () => {
        deps!();
        impl < U , B > ToInt < u64 > for UInt < U , B > where U : Unsigned , B : Bit , { # [inline] fn to_int () -> u64 { Self :: U64 } const INT : u64 = Self :: U64 ; }
    };
}

impl_523!();