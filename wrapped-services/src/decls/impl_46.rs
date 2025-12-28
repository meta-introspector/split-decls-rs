macro_rules! deps {
    () => {
        ServiceContext!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        unsafe impl Send for ServiceContext { }
    };
}

impl_46!()