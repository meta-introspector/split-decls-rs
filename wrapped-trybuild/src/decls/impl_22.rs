macro_rules! deps {
    () => {
        TestCases!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl RefUnwindSafe for TestCases { }
    };
}

impl_22!()