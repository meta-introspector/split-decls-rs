macro_rules! deps {
    () => {
        Shared!();
        Aliasing!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl Aliasing for Shared { const IS_EXCLUSIVE : bool = false ; }
    };
}

impl_321!()