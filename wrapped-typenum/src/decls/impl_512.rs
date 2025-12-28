macro_rules! deps {
    () => {
        ToInt!();
        UTerm!();
    };
}

macro_rules! impl_512 {
    () => {
        deps!();
        impl ToInt < usize > for UTerm { # [inline] fn to_int () -> usize { Self :: USIZE } const INT : usize = Self :: USIZE ; }
    };
}

impl_512!()