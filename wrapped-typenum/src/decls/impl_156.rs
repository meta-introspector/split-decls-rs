macro_rules! deps {
    () => {
        NInt!();
        NonZero!();
        Unsigned!();
        ToInt!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        # [cfg (feature = "i128")] impl < U > ToInt < i128 > for NInt < U > where U : Unsigned + NonZero , { # [inline] fn to_int () -> i128 { Self :: I128 } const INT : i128 = Self :: I128 ; }
    };
}

impl_156!()