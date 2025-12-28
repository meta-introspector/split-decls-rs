macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_1624 {
    () => {
        deps!();
        impl Integer for i8 { }
    };
}

impl_1624!();