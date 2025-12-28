macro_rules! deps {
    () => {
        NonZero!();
        PInt!();
        Unsigned!();
        ToInt!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < U > ToInt < i16 > for PInt < U > where U : Unsigned + NonZero , { # [inline] fn to_int () -> i16 { Self :: I16 } const INT : i16 = Self :: I16 ; }
    };
}

impl_158!();