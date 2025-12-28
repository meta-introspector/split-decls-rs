macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl Eq for HSTRING { }
    };
}

impl_35!();