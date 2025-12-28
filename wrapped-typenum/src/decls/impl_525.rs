macro_rules! deps {
    () => {
        UInt!();
        Bit!();
        Unsigned!();
        ToInt!();
    };
}

macro_rules! impl_525 {
    () => {
        deps!();
        # [cfg (feature = "i128")] impl < U , B > ToInt < u128 > for UInt < U , B > where U : Unsigned , B : Bit , { # [inline] fn to_int () -> u128 { Self :: U128 } const INT : u128 = Self :: U128 ; }
    };
}

impl_525!()