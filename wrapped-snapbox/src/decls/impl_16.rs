macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Eq for Error { }
    };
}

impl_16!()