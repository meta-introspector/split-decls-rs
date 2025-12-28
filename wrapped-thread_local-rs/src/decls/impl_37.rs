macro_rules! deps {
    () => {
        ThreadLocal!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T : Send + UnwindSafe > UnwindSafe for ThreadLocal < T > { }
    };
}

impl_37!();