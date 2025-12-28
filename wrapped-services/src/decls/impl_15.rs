macro_rules! deps {
    () => {
        ServiceContext!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        unsafe impl Send for ServiceContext { }
    };
}

impl_15!()