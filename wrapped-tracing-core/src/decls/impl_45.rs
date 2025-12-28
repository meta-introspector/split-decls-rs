macro_rules! deps {
    () => {
        Identifier!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl Eq for Identifier { }
    };
}

impl_45!()