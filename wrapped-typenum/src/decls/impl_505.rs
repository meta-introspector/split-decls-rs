macro_rules! deps {
    () => {
        ToInt!();
        UTerm!();
    };
}

macro_rules! impl_505 {
    () => {
        deps!();
        impl ToInt < i64 > for UTerm { # [inline] fn to_int () -> i64 { Self :: I64 } const INT : i64 = Self :: I64 ; }
    };
}

impl_505!();