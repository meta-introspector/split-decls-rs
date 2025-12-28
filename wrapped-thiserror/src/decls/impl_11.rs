macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Sealed for dyn Error + '_ { }
    };
}

impl_11!();