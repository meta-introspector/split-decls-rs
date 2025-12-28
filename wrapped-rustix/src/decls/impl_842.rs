macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_842 {
    () => {
        deps!();
        impl Integer for i16 { }
    };
}

impl_842!()