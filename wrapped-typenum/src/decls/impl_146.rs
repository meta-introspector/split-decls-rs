macro_rules! deps {
    () => {
        ToInt!();
        Z0!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl ToInt < i16 > for Z0 { # [inline] fn to_int () -> i16 { Self :: I16 } const INT : i16 = Self :: I16 ; }
    };
}

impl_146!()