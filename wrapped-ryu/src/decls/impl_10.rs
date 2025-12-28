macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Float for f64 { }
    };
}

impl_10!()