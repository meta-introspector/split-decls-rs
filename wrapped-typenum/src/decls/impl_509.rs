macro_rules! deps {
    () => {
        ToInt!();
        UTerm!();
    };
}

macro_rules! impl_509 {
    () => {
        deps!();
        impl ToInt < u16 > for UTerm { # [inline] fn to_int () -> u16 { Self :: U16 } const INT : u16 = Self :: U16 ; }
    };
}

impl_509!();