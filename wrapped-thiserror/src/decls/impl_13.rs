macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Sealed for dyn Error + Send + Sync + '_ { }
    };
}

impl_13!()