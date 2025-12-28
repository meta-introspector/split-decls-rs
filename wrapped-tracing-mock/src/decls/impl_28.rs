macro_rules! deps {
    () => {
        ExpectedValue!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Eq for ExpectedValue { }
    };
}

impl_28!();