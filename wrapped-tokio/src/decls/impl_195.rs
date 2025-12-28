macro_rules! deps {
    () => {
        AtomicUsize!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        unsafe impl Sync for AtomicUsize { }
    };
}

impl_195!()