macro_rules! deps {
    () => {
        ToInt!();
        UTerm!();
    };
}

macro_rules! impl_510 {
    () => {
        deps!();
        impl ToInt < u32 > for UTerm { # [inline] fn to_int () -> u32 { Self :: U32 } const INT : u32 = Self :: U32 ; }
    };
}

impl_510!()