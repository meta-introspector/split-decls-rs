macro_rules! deps {
    () => {
        ParseBuffer!();
    };
}

macro_rules! impl_502 {
    () => {
        deps!();
        impl < 'a > RefUnwindSafe for ParseBuffer < 'a > { }
    };
}

impl_502!();