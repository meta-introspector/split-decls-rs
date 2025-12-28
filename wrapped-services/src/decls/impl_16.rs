macro_rules! deps {
    () => {
        ServiceContext!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        unsafe impl Sync for ServiceContext { }
    };
}

impl_16!()