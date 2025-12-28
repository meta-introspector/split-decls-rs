macro_rules! deps {
    () => {
        AtomicU32!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        unsafe impl Sync for AtomicU32 { }
    };
}

impl_183!();