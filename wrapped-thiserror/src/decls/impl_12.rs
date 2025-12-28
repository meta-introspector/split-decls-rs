macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Sealed for dyn Error + Send + '_ { }
    };
}

impl_12!()