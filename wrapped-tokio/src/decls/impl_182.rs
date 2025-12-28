macro_rules! deps {
    () => {
        AtomicU32!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        unsafe impl Send for AtomicU32 { }
    };
}

impl_182!()