macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_846 {
    () => {
        deps!();
        impl Integer for u16 { }
    };
}

impl_846!();