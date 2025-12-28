macro_rules! deps {
    () => {
        ToInt!();
        NonZero!();
        PInt!();
        Unsigned!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < U > ToInt < isize > for PInt < U > where U : Unsigned + NonZero , { # [inline] fn to_int () -> isize { Self :: ISIZE } const INT : isize = Self :: ISIZE ; }
    };
}

impl_161!()