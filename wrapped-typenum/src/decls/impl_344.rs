macro_rules! deps {
    () => {
        Bit!();
        UInt!();
        Unsigned!();
        NonZero!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl < U : Unsigned , B : Bit > NonZero for UInt < U , B > { }
    };
}

impl_344!()