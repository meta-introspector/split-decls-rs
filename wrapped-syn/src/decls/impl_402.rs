macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! impl_402 {
    () => {
        deps!();
        impl Eq for Lifetime { }
    };
}

impl_402!()