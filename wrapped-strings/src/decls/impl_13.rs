macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Eq for BSTR { }
    };
}

impl_13!();