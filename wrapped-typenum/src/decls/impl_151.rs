macro_rules! deps {
    () => {
        NonZero!();
        Unsigned!();
        ToInt!();
        NInt!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < U > ToInt < i8 > for NInt < U > where U : Unsigned + NonZero , { # [inline] fn to_int () -> i8 { Self :: I8 } const INT : i8 = Self :: I8 ; }
    };
}

impl_151!();