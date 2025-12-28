macro_rules! deps {
    () => {
        ServiceContext!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        unsafe impl Sync for ServiceContext { }
    };
}

impl_47!()