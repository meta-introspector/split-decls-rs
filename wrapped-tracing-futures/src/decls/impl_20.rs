macro_rules! deps {
    () => {
        Instrument!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T : Sized > Instrument for T { }
    };
}

impl_20!();