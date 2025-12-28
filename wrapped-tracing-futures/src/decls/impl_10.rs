macro_rules! deps {
    () => {
        Instrument!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T : Sized > Instrument for T { }
    };
}

impl_10!()