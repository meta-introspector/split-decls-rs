macro_rules! deps {
    () => {
        ToInt!();
        Unsigned!();
        UInt!();
        Bit!();
    };
}

macro_rules! impl_518 {
    () => {
        deps!();
        impl < U , B > ToInt < isize > for UInt < U , B > where U : Unsigned , B : Bit , { # [inline] fn to_int () -> isize { Self :: ISIZE } const INT : isize = Self :: ISIZE ; }
    };
}

impl_518!();