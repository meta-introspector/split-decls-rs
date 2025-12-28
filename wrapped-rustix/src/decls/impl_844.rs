macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_844 {
    () => {
        deps!();
        impl Integer for i64 { }
    };
}

impl_844!()