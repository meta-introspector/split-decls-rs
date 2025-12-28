macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_1628 {
    () => {
        deps!();
        impl Integer for u8 { }
    };
}

impl_1628!()