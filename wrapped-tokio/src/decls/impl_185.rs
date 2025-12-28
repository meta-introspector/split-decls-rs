macro_rules! deps {
    () => {
        AtomicU32!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl panic :: UnwindSafe for AtomicU32 { }
    };
}

impl_185!();