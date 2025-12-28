macro_rules! deps {
    () => {
        ToInt!();
        UTerm!();
    };
}

macro_rules! impl_508 {
    () => {
        deps!();
        impl ToInt < u8 > for UTerm { # [inline] fn to_int () -> u8 { Self :: U8 } const INT : u8 = Self :: U8 ; }
    };
}

impl_508!()