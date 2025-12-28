macro_rules! deps {
    () => {
        TestCases!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl std :: panic :: RefUnwindSafe for TestCases { }
    };
}

impl_40!();