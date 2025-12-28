macro_rules! deps {
    () => {
        Identifier!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Eq for Identifier { }
    };
}

impl_50!();