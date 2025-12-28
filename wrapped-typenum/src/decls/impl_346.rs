macro_rules! deps {
    () => {
        PowerOfTwo!();
        UInt!();
        UTerm!();
        B1!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl PowerOfTwo for UInt < UTerm , B1 > { }
    };
}

impl_346!();