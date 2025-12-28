macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_1629 {
    () => {
        deps!();
        impl Integer for u16 { }
    };
}

impl_1629!();