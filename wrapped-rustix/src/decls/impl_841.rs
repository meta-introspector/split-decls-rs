macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_841 {
    () => {
        deps!();
        impl Integer for i8 { }
    };
}

impl_841!()