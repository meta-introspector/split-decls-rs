macro_rules! deps {
    () => {
        Bit!();
        UInt!();
        ToInt!();
        Unsigned!();
    };
}

macro_rules! impl_525 {
    () => {
        deps!();
        # [cfg (feature = "i128")] impl < U , B > ToInt < u128 > for UInt < U , B > where U : Unsigned , B : Bit , { # [inline] fn to_int () -> u128 { Self :: U128 } const INT : u128 = Self :: U128 ; }
    };
}

impl_525!();