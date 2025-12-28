macro_rules! deps {
    () => {
        ToInt!();
        Z0!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl ToInt < i64 > for Z0 { # [inline] fn to_int () -> i64 { Self :: I64 } const INT : i64 = Self :: I64 ; }
    };
}

impl_148!()