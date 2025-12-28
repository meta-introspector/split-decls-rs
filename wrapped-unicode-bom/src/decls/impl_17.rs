macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Eq for Bom { }
    };
}

impl_17!();