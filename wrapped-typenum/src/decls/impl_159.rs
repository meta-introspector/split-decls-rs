macro_rules! deps {
    () => {
        ToInt!();
        NonZero!();
        Unsigned!();
        PInt!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < U > ToInt < i32 > for PInt < U > where U : Unsigned + NonZero , { # [inline] fn to_int () -> i32 { Self :: I32 } const INT : i32 = Self :: I32 ; }
    };
}

impl_159!();