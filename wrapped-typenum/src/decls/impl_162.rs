macro_rules! deps {
    () => {
        NonZero!();
        ToInt!();
        Unsigned!();
        PInt!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        # [cfg (feature = "i128")] impl < U > ToInt < i128 > for PInt < U > where U : Unsigned + NonZero , { # [inline] fn to_int () -> i128 { Self :: I128 } const INT : i128 = Self :: I128 ; }
    };
}

impl_162!();