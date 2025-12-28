macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_845 {
    () => {
        deps!();
        impl Integer for u8 { }
    };
}

impl_845!();