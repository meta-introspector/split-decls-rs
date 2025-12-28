macro_rules! deps {
    () => {
        Z0!();
        ToInt!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        # [cfg (feature = "i128")] impl ToInt < i128 > for Z0 { # [inline] fn to_int () -> i128 { Self :: I128 } const INT : i128 = Self :: I128 ; }
    };
}

impl_150!();