macro_rules! deps {
    () => {
        ToInt!();
        UTerm!();
    };
}

macro_rules! impl_506 {
    () => {
        deps!();
        impl ToInt < isize > for UTerm { # [inline] fn to_int () -> isize { Self :: ISIZE } const INT : isize = Self :: ISIZE ; }
    };
}

impl_506!();