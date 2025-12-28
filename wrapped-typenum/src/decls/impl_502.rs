macro_rules! deps {
    () => {
        UTerm!();
        ToInt!();
    };
}

macro_rules! impl_502 {
    () => {
        deps!();
        impl ToInt < i8 > for UTerm { # [inline] fn to_int () -> i8 { Self :: I8 } const INT : i8 = Self :: I8 ; }
    };
}

impl_502!()