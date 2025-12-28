macro_rules! deps {
    () => {
        ToInt!();
        PInt!();
        Unsigned!();
        NonZero!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < U > ToInt < i8 > for PInt < U > where U : Unsigned + NonZero , { # [inline] fn to_int () -> i8 { Self :: I8 } const INT : i8 = Self :: I8 ; }
    };
}

impl_157!()