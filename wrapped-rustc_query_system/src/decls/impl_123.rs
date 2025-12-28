macro_rules! deps {
    () => {
        StableHashingContext!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < 'ctx > rustc_abi :: HashStableContext for StableHashingContext < 'ctx > { }
    };
}

impl_123!();