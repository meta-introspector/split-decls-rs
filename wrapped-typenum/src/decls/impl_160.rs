macro_rules! deps {
    () => {
        ToInt!();
        NonZero!();
        Unsigned!();
        PInt!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < U > ToInt < i64 > for PInt < U > where U : Unsigned + NonZero , { # [inline] fn to_int () -> i64 { Self :: I64 } const INT : i64 = Self :: I64 ; }
    };
}

impl_160!();