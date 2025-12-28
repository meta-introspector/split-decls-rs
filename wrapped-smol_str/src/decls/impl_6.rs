macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Eq for SmolStr { }
    };
}

impl_6!()