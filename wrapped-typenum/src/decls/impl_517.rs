macro_rules! deps {
    () => {
        ToInt!();
        Unsigned!();
        UInt!();
        Bit!();
    };
}

macro_rules! impl_517 {
    () => {
        deps!();
        impl < U , B > ToInt < i64 > for UInt < U , B > where U : Unsigned , B : Bit , { # [inline] fn to_int () -> i64 { Self :: I64 } const INT : i64 = Self :: I64 ; }
    };
}

impl_517!()