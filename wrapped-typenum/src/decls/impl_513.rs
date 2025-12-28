macro_rules! deps {
    () => {
        ToInt!();
        UTerm!();
    };
}

macro_rules! impl_513 {
    () => {
        deps!();
        # [cfg (feature = "i128")] impl ToInt < u128 > for UTerm { # [inline] fn to_int () -> u128 { Self :: U128 } const INT : u128 = Self :: U128 ; }
    };
}

impl_513!();