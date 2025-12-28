macro_rules! deps {
    () => {
        UInt!();
        Unsigned!();
        Bit!();
        NonZero!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl < U : Unsigned , B : Bit > NonZero for UInt < U , B > { }
    };
}

impl_344!();