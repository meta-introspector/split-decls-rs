macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_847 {
    () => {
        deps!();
        impl Integer for u32 { }
    };
}

impl_847!();