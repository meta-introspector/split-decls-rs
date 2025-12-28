macro_rules! deps {
    () => {
        Service!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        unsafe impl Sync for Service < '_ > { }
    };
}

impl_40!();