macro_rules! deps {
    () => {
        NonZero!();
        NInt!();
        ToInt!();
        Unsigned!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < U > ToInt < i16 > for NInt < U > where U : Unsigned + NonZero , { # [inline] fn to_int () -> i16 { Self :: I16 } const INT : i16 = Self :: I16 ; }
    };
}

impl_152!()