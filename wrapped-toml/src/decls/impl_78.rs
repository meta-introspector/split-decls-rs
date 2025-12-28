macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < T : Sealed + ? Sized > Sealed for & T { }
    };
}

impl_78!();