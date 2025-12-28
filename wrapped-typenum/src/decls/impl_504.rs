macro_rules! deps {
    () => {
        ToInt!();
        UTerm!();
    };
}

macro_rules! impl_504 {
    () => {
        deps!();
        impl ToInt < i32 > for UTerm { # [inline] fn to_int () -> i32 { Self :: I32 } const INT : i32 = Self :: I32 ; }
    };
}

impl_504!();