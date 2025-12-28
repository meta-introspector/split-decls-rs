macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T : Display + ? Sized > Sealed for & T { }
    };
}

impl_21!();