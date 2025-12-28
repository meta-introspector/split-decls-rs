macro_rules! deps {
    () => {
        AtomicU16!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        unsafe impl Sync for AtomicU16 { }
    };
}

impl_174!();