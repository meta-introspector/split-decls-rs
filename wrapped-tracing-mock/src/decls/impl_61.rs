macro_rules! deps {
    () => {
        ExpectedId!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl Eq for ExpectedId { }
    };
}

impl_61!()