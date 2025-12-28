macro_rules! deps {
    () => {
        Bit!();
        UInt!();
        Unsigned!();
        ToInt!();
    };
}

macro_rules! impl_522 {
    () => {
        deps!();
        impl < U , B > ToInt < u32 > for UInt < U , B > where U : Unsigned , B : Bit , { # [inline] fn to_int () -> u32 { Self :: U32 } const INT : u32 = Self :: U32 ; }
    };
}

impl_522!()