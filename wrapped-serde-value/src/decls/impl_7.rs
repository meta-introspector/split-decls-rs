macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Eq for Value { }
    };
}

impl_7!()