macro_rules! deps {
    () => {
        ToInt!();
        Unsigned!();
        UInt!();
        Bit!();
    };
}

macro_rules! impl_521 {
    () => {
        deps!();
        impl < U , B > ToInt < u16 > for UInt < U , B > where U : Unsigned , B : Bit , { # [inline] fn to_int () -> u16 { Self :: U16 } const INT : u16 = Self :: U16 ; }
    };
}

impl_521!();