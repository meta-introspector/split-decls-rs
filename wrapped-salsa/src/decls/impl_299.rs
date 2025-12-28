macro_rules! deps {
    () => {
        Coordinate!();
    };
}

macro_rules! impl_299 {
    () => {
        deps!();
        impl RefUnwindSafe for Coordinate { }
    };
}

impl_299!();