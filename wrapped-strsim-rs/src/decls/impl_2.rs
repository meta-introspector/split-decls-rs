macro_rules! deps {
    () => {
        StrSimError!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Error for StrSimError { }
    };
}

impl_2!();