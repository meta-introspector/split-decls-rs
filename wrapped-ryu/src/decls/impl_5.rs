macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Copy for Buffer { }
    };
}

impl_5!()