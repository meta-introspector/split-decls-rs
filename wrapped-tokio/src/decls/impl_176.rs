macro_rules! deps {
    () => {
        AtomicU16!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl panic :: UnwindSafe for AtomicU16 { }
    };
}

impl_176!()