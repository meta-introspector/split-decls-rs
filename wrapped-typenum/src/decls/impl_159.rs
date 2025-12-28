macro_rules! deps {
    () => {
        Unsigned!();
        NonZero!();
        PInt!();
        ToInt!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < U > ToInt < i32 > for PInt < U > where U : Unsigned + NonZero , { # [inline] fn to_int () -> i32 { Self :: I32 } const INT : i32 = Self :: I32 ; }
    };
}

impl_159!()