macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_1631 {
    () => {
        deps!();
        impl Integer for u64 { }
    };
}

impl_1631!();