macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_1627 {
    () => {
        deps!();
        impl Integer for i64 { }
    };
}

impl_1627!();