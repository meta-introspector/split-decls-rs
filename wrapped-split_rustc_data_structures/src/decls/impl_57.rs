macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < T > Frozen < T > { pub fn freeze (val : T) -> Self { Frozen (val) } }
    };
}

impl_57!()