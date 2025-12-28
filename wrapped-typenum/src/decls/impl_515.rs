macro_rules! deps {
    () => {
        ToInt!();
        Unsigned!();
        Bit!();
        UInt!();
    };
}

macro_rules! impl_515 {
    () => {
        deps!();
        impl < U , B > ToInt < i16 > for UInt < U , B > where U : Unsigned , B : Bit , { # [inline] fn to_int () -> i16 { Self :: I16 } const INT : i16 = Self :: I16 ; }
    };
}

impl_515!()