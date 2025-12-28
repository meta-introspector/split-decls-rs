macro_rules! deps {
    () => {
        AtomicU16!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl panic :: RefUnwindSafe for AtomicU16 { }
    };
}

impl_175!()