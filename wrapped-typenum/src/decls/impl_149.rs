macro_rules! deps {
    () => {
        ToInt!();
        Z0!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl ToInt < isize > for Z0 { # [inline] fn to_int () -> isize { Self :: ISIZE } const INT : isize = Self :: ISIZE ; }
    };
}

impl_149!()