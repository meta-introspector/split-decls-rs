macro_rules! deps {
    () => {
        Unsigned!();
        ToInt!();
        Bit!();
        UInt!();
    };
}

macro_rules! impl_519 {
    () => {
        deps!();
        # [cfg (feature = "i128")] impl < U , B > ToInt < i128 > for UInt < U , B > where U : Unsigned , B : Bit , { # [inline] fn to_int () -> i128 { Self :: I128 } const INT : i128 = Self :: I128 ; }
    };
}

impl_519!();