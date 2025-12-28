macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_848 {
    () => {
        deps!();
        impl Integer for u64 { }
    };
}

impl_848!();