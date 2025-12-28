macro_rules! deps {
    () => {
        ToInt!();
        Unsigned!();
        Bit!();
        UInt!();
    };
}

macro_rules! impl_524 {
    () => {
        deps!();
        impl < U , B > ToInt < usize > for UInt < U , B > where U : Unsigned , B : Bit , { # [inline] fn to_int () -> usize { Self :: USIZE } const INT : usize = Self :: USIZE ; }
    };
}

impl_524!();