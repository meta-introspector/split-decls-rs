macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_1630 {
    () => {
        deps!();
        impl Integer for u32 { }
    };
}

impl_1630!();