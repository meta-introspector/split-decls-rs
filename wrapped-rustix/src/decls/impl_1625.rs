macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_1625 {
    () => {
        deps!();
        impl Integer for i16 { }
    };
}

impl_1625!();