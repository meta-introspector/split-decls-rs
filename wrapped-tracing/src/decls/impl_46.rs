macro_rules! deps {
    () => {
        Instrument!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < T : Sized > Instrument for T { }
    };
}

impl_46!()