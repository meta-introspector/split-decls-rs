macro_rules! deps {
    () => {
        AtomicUsize!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl panic :: UnwindSafe for AtomicUsize { }
    };
}

impl_197!();