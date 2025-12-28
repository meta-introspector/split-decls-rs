macro_rules! deps {
    () => {
        ParseBuffer!();
    };
}

macro_rules! impl_501 {
    () => {
        deps!();
        impl < 'a > UnwindSafe for ParseBuffer < 'a > { }
    };
}

impl_501!()