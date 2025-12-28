macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_843 {
    () => {
        deps!();
        impl Integer for i32 { }
    };
}

impl_843!();