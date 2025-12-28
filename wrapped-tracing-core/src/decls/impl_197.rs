macro_rules! deps {
    () => {
        Metadata!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl Eq for Metadata < '_ > { }
    };
}

impl_197!()