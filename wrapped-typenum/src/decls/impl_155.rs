macro_rules! deps {
    () => {
        NonZero!();
        ToInt!();
        Unsigned!();
        NInt!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < U > ToInt < isize > for NInt < U > where U : Unsigned + NonZero , { # [inline] fn to_int () -> isize { Self :: ISIZE } const INT : isize = Self :: ISIZE ; }
    };
}

impl_155!();