macro_rules! deps {
    () => {
        FatalError!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl ! Send for FatalError { }
    };
}

impl_225!()