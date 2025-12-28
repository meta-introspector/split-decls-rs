macro_rules! deps {
    () => {
        Bit!();
        UInt!();
        ToInt!();
        Unsigned!();
    };
}

macro_rules! impl_516 {
    () => {
        deps!();
        impl < U , B > ToInt < i32 > for UInt < U , B > where U : Unsigned , B : Bit , { # [inline] fn to_int () -> i32 { Self :: I32 } const INT : i32 = Self :: I32 ; }
    };
}

impl_516!();