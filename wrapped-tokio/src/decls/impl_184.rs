macro_rules! deps {
    () => {
        AtomicU32!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl panic :: RefUnwindSafe for AtomicU32 { }
    };
}

impl_184!()