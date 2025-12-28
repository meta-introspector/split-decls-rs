macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Type for ! { }
    };
}

impl_33!();