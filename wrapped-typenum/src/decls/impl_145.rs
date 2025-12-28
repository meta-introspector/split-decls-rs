macro_rules! deps {
    () => {
        ToInt!();
        Z0!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl ToInt < i8 > for Z0 { # [inline] fn to_int () -> i8 { Self :: I8 } const INT : i8 = Self :: I8 ; }
    };
}

impl_145!();