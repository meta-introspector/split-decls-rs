macro_rules! deps {
    () => {
        B0!();
        Zero!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Zero for B0 { }
    };
}

impl_6!()