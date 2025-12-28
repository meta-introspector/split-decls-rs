macro_rules! deps {
    () => {
        CachedThreadLocal!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < T : Send + UnwindSafe > UnwindSafe for CachedThreadLocal < T > { }
    };
}

impl_7!();