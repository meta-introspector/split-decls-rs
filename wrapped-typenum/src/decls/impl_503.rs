macro_rules! deps {
    () => {
        ToInt!();
        UTerm!();
    };
}

macro_rules! impl_503 {
    () => {
        deps!();
        impl ToInt < i16 > for UTerm { # [inline] fn to_int () -> i16 { Self :: I16 } const INT : i16 = Self :: I16 ; }
    };
}

impl_503!()