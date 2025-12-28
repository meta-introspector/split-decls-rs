macro_rules! deps {
    () => {
        TestCases!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl RefUnwindSafe for TestCases { }
    };
}

impl_183!()