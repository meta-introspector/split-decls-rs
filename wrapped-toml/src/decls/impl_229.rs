macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl < T : Sealed + ? Sized > Sealed for & T { }
    };
}

impl_229!()