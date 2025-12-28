macro_rules! deps {
    () => {
        ToInt!();
        Z0!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl ToInt < i32 > for Z0 { # [inline] fn to_int () -> i32 { Self :: I32 } const INT : i32 = Self :: I32 ; }
    };
}

impl_147!();