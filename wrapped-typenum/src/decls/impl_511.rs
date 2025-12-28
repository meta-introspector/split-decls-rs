macro_rules! deps {
    () => {
        ToInt!();
        UTerm!();
    };
}

macro_rules! impl_511 {
    () => {
        deps!();
        impl ToInt < u64 > for UTerm { # [inline] fn to_int () -> u64 { Self :: U64 } const INT : u64 = Self :: U64 ; }
    };
}

impl_511!()