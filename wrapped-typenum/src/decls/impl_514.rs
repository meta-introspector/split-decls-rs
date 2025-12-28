macro_rules! deps {
    () => {
        Unsigned!();
        Bit!();
        ToInt!();
        UInt!();
    };
}

macro_rules! impl_514 {
    () => {
        deps!();
        impl < U , B > ToInt < i8 > for UInt < U , B > where U : Unsigned , B : Bit , { # [inline] fn to_int () -> i8 { Self :: I8 } const INT : i8 = Self :: I8 ; }
    };
}

impl_514!()