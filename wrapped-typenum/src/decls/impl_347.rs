macro_rules! deps {
    () => {
        Unsigned!();
        PowerOfTwo!();
        UInt!();
        B0!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl < U : Unsigned + PowerOfTwo > PowerOfTwo for UInt < U , B0 > { }
    };
}

impl_347!();