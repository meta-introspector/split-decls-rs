macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_1626 {
    () => {
        deps!();
        impl Integer for i32 { }
    };
}

impl_1626!()