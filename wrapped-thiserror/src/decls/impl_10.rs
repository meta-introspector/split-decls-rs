macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T : Error > Sealed for T { }
    };
}

impl_10!();