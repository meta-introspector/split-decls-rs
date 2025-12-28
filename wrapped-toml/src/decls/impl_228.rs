macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl Sealed for String { }
    };
}

impl_228!();