macro_rules! deps {
    () => {
        PowerOfTwo!();
        B0!();
        UInt!();
        Unsigned!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl < U : Unsigned + PowerOfTwo > PowerOfTwo for UInt < U , B0 > { }
    };
}

impl_347!()