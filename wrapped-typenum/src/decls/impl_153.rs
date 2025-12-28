macro_rules! deps {
    () => {
        NInt!();
        ToInt!();
        Unsigned!();
        NonZero!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < U > ToInt < i32 > for NInt < U > where U : Unsigned + NonZero , { # [inline] fn to_int () -> i32 { Self :: I32 } const INT : i32 = Self :: I32 ; }
    };
}

impl_153!()