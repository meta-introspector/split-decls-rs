macro_rules! deps {
    () => {
        FixupContext!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl Copy for FixupContext { }
    };
}

impl_281!();