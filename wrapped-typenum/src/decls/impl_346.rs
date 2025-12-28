macro_rules! deps {
    () => {
        PowerOfTwo!();
        B1!();
        UTerm!();
        UInt!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl PowerOfTwo for UInt < UTerm , B1 > { }
    };
}

impl_346!()