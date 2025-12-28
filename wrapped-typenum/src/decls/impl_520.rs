macro_rules! deps {
    () => {
        ToInt!();
        Unsigned!();
        Bit!();
        UInt!();
    };
}

macro_rules! impl_520 {
    () => {
        deps!();
        impl < U , B > ToInt < u8 > for UInt < U , B > where U : Unsigned , B : Bit , { # [inline] fn to_int () -> u8 { Self :: U8 } const INT : u8 = Self :: U8 ; }
    };
}

impl_520!()