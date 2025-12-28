macro_rules! deps {
    () => {
        AtomicUsize!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl panic :: RefUnwindSafe for AtomicUsize { }
    };
}

impl_196!();