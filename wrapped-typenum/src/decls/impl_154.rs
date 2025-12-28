macro_rules! deps {
    () => {
        NonZero!();
        NInt!();
        ToInt!();
        Unsigned!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < U > ToInt < i64 > for NInt < U > where U : Unsigned + NonZero , { # [inline] fn to_int () -> i64 { Self :: I64 } const INT : i64 = Self :: I64 ; }
    };
}

impl_154!()