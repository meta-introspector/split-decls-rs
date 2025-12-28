macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T : Error + ? Sized > Sealed for T { }
    };
}

impl_29!()