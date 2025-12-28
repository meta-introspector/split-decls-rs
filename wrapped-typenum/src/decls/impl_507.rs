macro_rules! deps {
    () => {
        UTerm!();
        ToInt!();
    };
}

macro_rules! impl_507 {
    () => {
        deps!();
        # [cfg (feature = "i128")] impl ToInt < i128 > for UTerm { # [inline] fn to_int () -> i128 { Self :: I128 } const INT : i128 = Self :: I128 ; }
    };
}

impl_507!();