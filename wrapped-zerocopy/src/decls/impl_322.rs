macro_rules! deps {
    () => {
        Reference!();
        Shared!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl Reference for Shared { }
    };
}

impl_322!();