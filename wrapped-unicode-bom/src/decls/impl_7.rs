macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Eq for Bom { }
    };
}

impl_7!()