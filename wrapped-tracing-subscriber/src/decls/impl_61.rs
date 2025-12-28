macro_rules! deps {
    () => {
        FilterFn!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < F > From < F > for FilterFn < F > where F : Fn (& Metadata < '_ >) -> bool , { fn from (enabled : F) -> Self { Self :: new (enabled) } }
    };
}

impl_61!()